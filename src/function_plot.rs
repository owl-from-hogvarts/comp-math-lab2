use std::{cell::RefCell, ops::Range};

use iced::Element;
use plotters::{
    chart::ChartContext,
    coord::types::RangedCoordf32,
    prelude::{Cartesian2d, ChartBuilder},
    series::LineSeries,
    style::{Color, ShapeStyle},
};
use plotters_iced::{Chart, ChartWidget, DrawingBackend};
use protocol::{
    point::{Point, PointCoordinate},
    request::EquationModeRaw,
    response::{ComputeRootResponse, FunctionPointsResponse, InitialApproximationsResponse},
    TNumber,
};

use crate::UIMessage;

pub enum Payload {
    Single(EquationPlot),
    System(SystemOfEquationsPlot),
}

pub enum PlotMessage {
    InitialAppriximations(InitialApproximationsResponse),
    Data(usize, Payload),
}

#[derive(Debug, Clone)]
pub struct EquationPlot {
    pub computed_root: ComputeRootResponse,
    pub function_poins: FunctionPointsResponse,
}

#[derive(Debug, Clone)]
pub struct SystemOfEquationsPlot {
    pub computed_root: ComputeRootResponse,
    pub first_function_points: FunctionPointsResponse,
    pub second_function_points: FunctionPointsResponse,
}

#[derive(Debug, Clone)]
pub struct Selected {
    pub mode: EquationModeRaw,
    pub index: usize,
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
    /// area requires to build dimmy chart with dimmy coordinates
    /// first. And only then build actual chart with proper coordinates
    aspect_ratio: RefCell<f64>,

    // local state
    // using structs instead of enum to preserve state.
    // this allows to switch plot data without re-requesting it
    single: Vec<Option<EquationPlot>>,
    system: Vec<Option<SystemOfEquationsPlot>>,
}

struct FunctionPlotState<'a> {
    selection: Selected,
    state: &'a FunctionPlot,
}

impl FunctionPlot {
    pub fn new() -> Self {
        const EQUATION_AMOUNT_WITH_RESERVE: usize = 10;

        let mut single = Vec::new();
        single.resize(EQUATION_AMOUNT_WITH_RESERVE, None);

        let mut system = Vec::new();
        system.resize(EQUATION_AMOUNT_WITH_RESERVE, None);

        Self {
            initial_approximations: Default::default(),
            aspect_ratio: RefCell::new(f64::NAN),
            single,
            system,
        }
    }

    pub fn update(&mut self, message: PlotMessage) {
        match message {
            PlotMessage::InitialAppriximations(response) => {
                self.initial_approximations = Some(response)
            }
            PlotMessage::Data(index, payload) => match payload {
                Payload::Single(data) => {
                    self.single[index] = Some(data);
                }
                Payload::System(data) => {
                    self.system[index] = Some(data);
                }
            },
        }
    }

    pub fn view(&self, selection: Selected) -> Element<UIMessage> {
        let is_loading = self.initial_approximations.is_none()
            && match selection.mode {
                EquationModeRaw::SingleEquation => self.single[selection.index].is_none(),
                EquationModeRaw::SystemOfEquations => self.system[selection.index].is_none(),
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
const COORD_MARGIN_PERSENT: TNumber = 0.05;

impl<'a> Chart<UIMessage> for FunctionPlotState<'a> {
    // internal state, part of stateless widgets model
    type State = ();

    fn build_chart<DB: plotters::prelude::DrawingBackend>(
        &self,
        _state: &Self::State,
        builder: ChartBuilder<DB>,
    ) {
        use plotters::prelude::*;
        use plotters::style::full_palette::{GREEN, ORANGE};

        const POINT_SIZE: i32 = 5;

        let aspect_ratio = self.state.aspect_ratio.replace_with(|old| *old);

        let (function_points, second, computed_root) = match self.selection.mode {
            EquationModeRaw::SingleEquation => {
                let equation = self.state.single[self.selection.index]
                    .as_ref()
                    .expect("Should be some! Other wise loading should be displayed");
                (&equation.function_poins, None, equation.computed_root.root)
            }
            EquationModeRaw::SystemOfEquations => {
                let system = self.state.system[self.selection.index]
                    .as_ref()
                    .expect("Should be some! Other wise loading should be displayed");
                (
                    &system.first_function_points,
                    Some(system.second_function_points),
                    system.computed_root.root,
                )
            }
        };

        let x_range = function_points.0.build_range(PointCoordinate::X);
        let x_range_length = x_range.end - x_range.end;
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

        draw_series(&mut chart, &function_points.0, ORANGE.stroke_width(3));

        if let Some(second) = second {
            draw_series(&mut chart, &second.0, GREEN.stroke_width(3))
        }

        chart
            .draw_series(PointSeries::<_, _, Circle<_, _>, _>::new(
                [computed_root].iter().map(|point| (point.x, point.y)),
                POINT_SIZE,
                RED.filled(),
            ))
            .expect("could draw root point");
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
    let mut chart = builder
        .margin(MARGINS * 2)
        .x_label_area_size(20)
        .y_label_area_size(40)
        .build_cartesian_2d(
            with_coord_margin(x_range.clone(), COORD_MARGIN_PERSENT),
            with_coord_margin(y_range, COORD_MARGIN_PERSENT),
        )
        .expect("Could not configure chart!");

    chart
        .configure_mesh()
        .label_style(("noto sans", 16))
        .x_labels(5)
        .y_labels(5)
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
