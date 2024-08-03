use std::{cell::RefCell, ops::Range};

use iced::Element;
use plotters::{
    chart::ChartContext,
    coord::types::RangedCoordf32,
    prelude::{Cartesian2d, ChartBuilder},
    series::LineSeries,
    style::{Color, RGBColor, ShapeStyle},
};
use plotters_iced::{Chart, ChartWidget, DrawingBackend};
use protocol::{
    point::{Point, PointCoordinate},
    request::{EquationMode, EquationModeRaw, RequestPackage},
    response::{
        ComputeRootResponse, FunctionPointsResponse, InitialApproximationsResponse, ResponsePackage,
    },
    TNumber,
};

use crate::UIMessage;

#[derive(Debug, Clone, Default)]
pub struct EquationPlot {
    pub computed_root: Option<ComputeRootResponse>,
    pub function_points: Option<FunctionPointsResponse>,
}

impl EquationPlot {
    fn is_loading(&self) -> bool {
        /* self.computed_root.is_none() || */
        self.function_points.is_none()
    }
}

#[derive(Debug, Clone, Default)]
pub struct SystemOfEquationsPlot {
    pub computed_root: Option<ComputeRootResponse>,
    pub first_function_points: Option<FunctionPointsResponse>,
    pub second_function_points: Option<FunctionPointsResponse>,
}

impl SystemOfEquationsPlot {
    fn is_loading(&self) -> bool {
        self.computed_root.is_none()
            || self.first_function_points.is_none()
            || self.second_function_points.is_none()
    }
}

#[derive(Debug, Clone)]
pub struct Selected {
    pub mode: EquationModeRaw,
    pub index: usize,
}

impl From<EquationMode> for Selected {
    fn from(value: EquationMode) -> Self {
        match value {
            EquationMode::Single {
                equation_number, ..
            } => Selected {
                mode: EquationModeRaw::SingleEquation,
                index: equation_number as usize,
            },
            EquationMode::SystemOfEquations { system_number } => Selected {
                mode: EquationModeRaw::SystemOfEquations,
                index: system_number as usize,
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct FunctionPlot {
    // shared state: shared accross different equations and modes
    initial_approximations: Option<InitialApproximationsResponse>,
    /// `ratio = width / height`
    ///
    /// Ratio is obtained from DrawingArea object.
    /// `build_chart` method does not have access to the drawing area unless
    /// it builds chart with *some* coordinates. Accessing drawing
    /// area requires to build dummy chart with dummy coordinates
    /// first. And only then build actual chart with proper coordinates
    aspect_ratio: RefCell<f64>,

    // local state
    // using structs instead of enum to preserve state.
    // this allows to switch plot data without re-requesting it.
    // Have to preserve data between requests to arduino.
    // This is a good place for the goal. Data is aggregated here until ready.
    single: Vec<EquationPlot>,
    system: Vec<SystemOfEquationsPlot>,
}

struct FunctionPlotState<'a> {
    selection: Selected,
    state: &'a FunctionPlot,
}

impl FunctionPlot {
    pub fn new() -> Self {
        const EQUATION_AMOUNT_WITH_RESERVE: usize = 10;

        let mut single = Vec::new();
        single.resize(EQUATION_AMOUNT_WITH_RESERVE, Default::default());

        let mut system = Vec::new();
        system.resize(EQUATION_AMOUNT_WITH_RESERVE, Default::default());

        Self {
            initial_approximations: Default::default(),
            aspect_ratio: RefCell::new(f64::NAN),
            single,
            system,
        }
    }

    pub fn update(&mut self, request: &RequestPackage, response: ResponsePackage) {
        if let ResponsePackage::InitialApproximations(response) = response {
            self.initial_approximations = Some(response);
            return;
        }

        let selection = match request {
            RequestPackage::FunctionPoints { payload } => Selected {
                mode: payload.mode,
                index: payload.equation_number as usize,
            },
            RequestPackage::ComputeRoot { payload } => payload.mode.clone().into(),
            _ => unreachable!(),
        };

        match selection.mode {
            EquationModeRaw::SingleEquation => {
                let single = &mut self.single[selection.index];

                match response {
                    ResponsePackage::ComputeRoot(response) => single.computed_root = response.ok(),
                    ResponsePackage::FunctionPoints(response) => {
                        single.function_points = Some(response)
                    }
                    _ => unreachable!(),
                }
            }
            EquationModeRaw::SystemOfEquations => {
                let system = &mut self.system[selection.index];

                match response {
                    ResponsePackage::ComputeRoot(response) => system.computed_root = response.ok(),
                    ResponsePackage::FunctionPoints(response) => {
                        system.first_function_points = Some(response)
                    }
                    ResponsePackage::FunctionPointsSecond(response) => {
                        system.second_function_points = Some(response)
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    pub fn view(&self, selection: Selected) -> Element<UIMessage> {
        let is_loading = self.initial_approximations.is_none()
            || match selection.mode {
                EquationModeRaw::SingleEquation => self.single[selection.index].is_loading(),
                EquationModeRaw::SystemOfEquations => self.system[selection.index].is_loading(),
            };

        if is_loading {
            return "Loading...".into();
        }
        let state = FunctionPlotState {
            selection,
            state: &self,
        };

        ChartWidget::new(state).into()
    }
}

const MARGINS: i32 = 10;
const COORD_MARGIN_PERCENT: TNumber = 0.05;

impl<'a> Chart<UIMessage> for FunctionPlotState<'a> {
    // internal state, part of stateless widgets model
    type State = ();

    fn build_chart<DB: plotters::prelude::DrawingBackend>(
        &self,
        _state: &Self::State,
        builder: ChartBuilder<DB>,
    ) {
        use plotters::prelude::*;

        const POINT_SIZE: i32 = 5;

        let aspect_ratio = self.state.aspect_ratio.replace_with(|old| *old);

        let (function_points, second, computed_root) = match self.selection.mode {
            EquationModeRaw::SingleEquation => {
                let equation = &self.state.single[self.selection.index];
                (
                    &equation.function_points.unwrap(),
                    None,
                    equation.computed_root,
                )
            }
            EquationModeRaw::SystemOfEquations => {
                let system = &self.state.system[self.selection.index];
                (
                    &system.first_function_points.unwrap(),
                    Some(system.second_function_points),
                    system.computed_root,
                )
            }
        };

        let x_range = function_points.0.build_range(PointCoordinate::X);
        let x_range_length = x_range.end - x_range.start;
        let y_range_half_length = x_range_length / aspect_ratio as f32 / 2.;
        let y_range = -y_range_half_length..y_range_half_length;

        // expects should never trigger as effective iced backend
        // is not capable of producing errors
        let initial_approximations = self.state.initial_approximations.expect(
            "Chart should be not drawn if initial approximations have not been received yet!",
        );

        let mut chart = configure_chart(builder, x_range, y_range);

        draw_vertical_line(&mut chart, initial_approximations.left);
        draw_vertical_line(&mut chart, initial_approximations.right);

        draw_series(
            &mut chart,
            &function_points.0,
            RGBColor(0xfe, 0x80, 0x19).stroke_width(3),
        );

        if let Some(second) = second {
            draw_series(
                &mut chart,
                &second.unwrap().0,
                RGBColor(0x8e, 0xc0, 0x7c).stroke_width(3),
            )
        }

        if let Some(response) = computed_root {
            let computed_root = response.root;
            chart
                .draw_series(PointSeries::<_, _, Circle<_, _>, _>::new(
                    [computed_root].iter().map(|point| (point.x, point.y)),
                    POINT_SIZE,
                    RGBColor(0xb8, 0xbb, 0x26).filled(),
                ))
                .expect("could draw root point");
        }
    }

    fn draw_chart<DB: DrawingBackend>(
        &self,
        state: &Self::State,
        root: plotters::prelude::DrawingArea<DB, plotters::coord::Shift>,
    ) {
        let (width, height) = root.dim_in_pixel();
        let ratio = width as f64 / height as f64;
        self.state.aspect_ratio.replace(ratio);

        let builder = ChartBuilder::on(&root);
        self.build_chart(state, builder);
    }
}

fn with_coord_margin(range: Range<TNumber>, margin_persents: TNumber) -> Range<TNumber> {
    let length = range.end - range.start;
    let margin = length * margin_persents;
    (range.start - margin)..(range.end + margin)
}

fn configure_chart<'a, DB: DrawingBackend>(
    mut builder: ChartBuilder<'a, 'a, DB>,
    x_range: Range<TNumber>,
    y_range: Range<TNumber>,
) -> ChartContext<'a, DB, Cartesian2d<RangedCoordf32, RangedCoordf32>> {
    // colors taken from Gruvbox from here:
    // https://www.figma.com/community/file/840895380520234275
    let mut chart = builder
        .margin(MARGINS * 2)
        .x_label_area_size(60)
        .y_label_area_size(60)
        .build_cartesian_2d(
            with_coord_margin(x_range.clone(), COORD_MARGIN_PERCENT),
            with_coord_margin(y_range, COORD_MARGIN_PERCENT),
        )
        .expect("Could not configure chart!");

    chart
        .configure_mesh()
        .label_style(("noto sans", 16, &RGBColor(0xfb, 0xf1, 0xc7)))
        .bold_line_style(RGBColor(0x66, 0x5c, 0x54))
        .light_line_style(RGBColor(0x3c, 0x38, 0x36))
        .x_labels(25)
        .y_labels(20)
        .x_desc("X")
        .y_desc("Y")
        .draw()
        .expect("could draw mesh");

    chart
}

fn draw_vertical_line<DB: DrawingBackend>(
    chart: &mut ChartContext<'_, DB, Cartesian2d<RangedCoordf32, RangedCoordf32>>,
    x: TNumber,
) {
    use plotters::style::full_palette::BLUE;

    const VERTICAL_LINE_WIDTH: u32 = 3;

    chart
        .draw_series(LineSeries::new(
            [(x, chart.y_range().start), (x, chart.y_range().end)],
            BLUE.stroke_width(VERTICAL_LINE_WIDTH),
        ))
        .unwrap();
}

fn draw_series<DB: DrawingBackend>(
    chart: &mut ChartContext<'_, DB, Cartesian2d<RangedCoordf32, RangedCoordf32>>,
    function_points: &[Point],
    style: ShapeStyle,
) {
    let y_range = chart.y_range();
    let sub_paths = function_points.split(|point| {
        let y = point.get_coordinate(PointCoordinate::Y);
        !y_range.contains(&y)
    });

    for path in sub_paths {
        chart
            .draw_series(LineSeries::new(
                path.iter().map(|point| (point.x, point.y)),
                style,
            ))
            .expect("could draw function points");
    }
}

trait RangeBuilder {
    fn build_range(&self, coordinate: PointCoordinate) -> Range<TNumber>;
}

impl RangeBuilder for [Point] {
    fn build_range(&self, coordinate: PointCoordinate) -> Range<TNumber> {
        self.iter()
            .min_by(|&a, &b| {
                a.get_coordinate(coordinate)
                    .total_cmp(&b.get_coordinate(coordinate))
            })
            .expect("at least one point present")
            .get_coordinate(coordinate)
            ..self
                .iter()
                .max_by(|&a, &b| {
                    a.get_coordinate(coordinate)
                        .total_cmp(&b.get_coordinate(coordinate))
                })
                .expect("at least one point present")
                .get_coordinate(coordinate)
    }
}
