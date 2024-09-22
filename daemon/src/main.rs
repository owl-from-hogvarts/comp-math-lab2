use std::fmt::Debug;

use function_plot::FunctionPlot;
use iced::futures::channel::mpsc::{self, Sender};
use iced::theme::{self};
use iced::widget::text_input;
use iced::widget::{button, column};
use iced::widget::{pick_list, Row};
use iced::widget::{row, Text};
use iced::{command, Alignment, Length, Padding};
use iced::{widget::Column, Element, Settings};
use iced::{Application, Command};

use iced_aw::{tabs::Tabs, TabLabel};
use protocol::point::Point;
use protocol::request::payloads::ComputeRootPayload;
use protocol::request::{self, compute_method::Method, EquationModeRaw, RequestPackage};
use protocol::request::{Selection, SingleEquation};
use protocol::response::{ComputeRootResponse, ResponsePackage};
use protocol::TNumber;
use serial_port_thread::start_loop;

mod function_plot;
mod serial_port_thread;

// don't know which sice is appropriate
const CHANNEL_SIZE: usize = 100;

#[derive(Debug, Clone, Copy)]
enum UIMessage {
    // not interested in payload
    TabSelect(EquationModeRaw),
    MethodSelect(request::compute_method::Method),
    Epsilon(TNumber),
    SingleEquationSelect(u8),
    SystemOfEquationsSelect(u8),
    /// Request contains context, such as for which equation
    /// points were requested. This eliminates class of bugs
    /// related to incoherent app state between request and response.
    ResponseReceived(RequestPackage, ResponsePackage),
}

fn main() -> iced::Result {
    // spawn thread
    // thread will manage arduino
    // periodically poll arduino for approximations
    // serial_port thread communicates with main thread
    // by pair of mpsc channels
    // application listen for approximation change

    // thread is initialized from within `subscribe`

    ComputeRootUI::run(Settings::default())
}

#[derive(Debug)]
struct ComputeRootUI {
    // this structure contains everything required to represent ui
    // and to send request
    epsilon: TNumber,
    mode: EquationModeRaw,
    single_equation: SingleEquation,
    system_of_equations_number: u8,
    serial_port: Sender<RequestPackage>,
    plot: FunctionPlot,
}

impl ComputeRootUI {
    /// Get selection from current state
    fn build_selection(&self) -> Selection {
        Selection {
            mode: self.mode,
            index: match self.mode {
                EquationModeRaw::SingleEquation => self.single_equation.equation_number,
                EquationModeRaw::SystemOfEquations => self.system_of_equations_number,
            },
        }
    }

    fn build_compute_root_payload(&self) -> ComputeRootPayload {
        ComputeRootPayload {
            epsilon: self.epsilon,
            mode: match self.mode {
                EquationModeRaw::SingleEquation => {
                    request::EquationMode::Single(self.single_equation)
                }
                EquationModeRaw::SystemOfEquations => request::EquationMode::SystemOfEquations {
                    system_number: self.system_of_equations_number,
                },
            },
        }
    }
}

impl Application for ComputeRootUI {
    type Message = UIMessage;
    type Executor = iced::executor::Default;
    type Theme = iced::Theme;
    type Flags = ();

    fn title(&self) -> String {
        String::from("A cool application")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        let mut approx_changed = false;
        match message {
            UIMessage::TabSelect(mode) => self.mode = mode,
            UIMessage::MethodSelect(method) => self.single_equation.method = method,
            UIMessage::Epsilon(epsilon) => self.epsilon = epsilon,
            UIMessage::SingleEquationSelect(equation_number) => {
                self.single_equation.equation_number = equation_number
            }
            UIMessage::ResponseReceived(ref request, response) => {
                approx_changed = self.plot.has_intial_approximations_changed(&response);
                self.plot.update(request, response);
            }
            UIMessage::SystemOfEquationsSelect(system_number) => {
                self.system_of_equations_number = system_number
            }
        };

        // should not take too long to send single structure to the serial
        // port thread synchronously
        match message {
            UIMessage::TabSelect(_)
            | UIMessage::SingleEquationSelect(_)
            | UIMessage::SystemOfEquationsSelect(_) => {
                let request = RequestPackage::FunctionPoints {
                    payload: self.build_selection(),
                };

                self.serial_port
                    .try_send(request)
                    .expect("could request function points");
            }

            _ => (),
        };

        let should_update_root = approx_changed
            || match message {
                UIMessage::TabSelect(_)
                | UIMessage::MethodSelect(_)
                | UIMessage::Epsilon(_)
                | UIMessage::SystemOfEquationsSelect(_)
                | UIMessage::SingleEquationSelect(_) => true,
                _ => false,
            };

        if should_update_root {
            let payload = self.build_compute_root_payload();
            let request = RequestPackage::ComputeRoot { payload };

            self.serial_port
                .try_send(request)
                .expect("could request root");
        }

        Command::none()
        // todo!();
    }

    fn view(&self) -> Element<Self::Message> {
        // dbg!(self);

        let single_equations = ["x^2 + x + sin(x)", "ln(x) + 15"]
            .into_iter()
            .enumerate()
            .map(|(index, equation)| {
                let text_container = Column::new()
                    .width(Length::Fill)
                    .align_items(iced::Alignment::Center)
                    .push(equation);
                let mut item = button(text_container)
                    .padding(10)
                    .width(Length::Fill)
                    .on_press(UIMessage::SingleEquationSelect(index as u8));

                if index != self.single_equation.equation_number as usize {
                    item = item.style(theme::Button::Secondary);
                }

                item.into()
            });

        // Padding between tabs labels and tabs themselves
        let tabs_padding = Padding {
            top: 10.,
            ..Padding::ZERO
        };

        const ROW_SPACING: f32 = 7.;
        const COLUMN_SPACING: f32 = 10.;
        let single_equation_tab = Column::new()
            .push(
                Column::with_children(single_equations)
                    // width is considered as min_width by iced
                    .width(Length::Fixed(300.))
                    .spacing(COLUMN_SPACING)
                    .align_items(iced::Alignment::Center),
            )
            .width(Length::Fill)
            .padding(tabs_padding)
            .align_items(iced::Alignment::Center);

        let system_of_equations_tab = column!("1 - sin(x) / 2 - x", "0.7 - cos(y - 1) - y")
            .spacing(COLUMN_SPACING)
            .padding(tabs_padding)
            .width(Length::Fill)
            .align_items(iced::Alignment::Center);

        let tabs_descriptor = Tabs::new(UIMessage::TabSelect)
            .push(
                EquationModeRaw::SingleEquation,
                TabLabel::Text("Single equation".to_string()),
                single_equation_tab,
            )
            .push(
                EquationModeRaw::SystemOfEquations,
                TabLabel::Text("System of Equations".to_string()),
                system_of_equations_tab,
            );

        // should always be displayed
        let parameters_row = row!(row!(
            "Epsilon:",
            text_input("", &format!("{:.4}", self.epsilon))
                .on_input(|input| { UIMessage::Epsilon(input.parse().unwrap_or(self.epsilon)) })
        )
        .spacing(ROW_SPACING)
        .align_items(iced::Alignment::Center))
        .spacing(ROW_SPACING)
        .align_items(iced::Alignment::Center);

        let parameters_row = if let EquationModeRaw::SingleEquation = self.mode {
            parameters_row.push(
                row!(
                    "method:",
                    pick_list(
                        [Method::Chord, Method::Secant, Method::SimpleIterationSingle],
                        Some(self.single_equation.method),
                        |method| UIMessage::MethodSelect(method)
                    )
                )
                .spacing(ROW_SPACING)
                .align_items(iced::Alignment::Center),
            )
        } else {
            parameters_row
        };

        let selection = self.build_selection();
        let maybe_compute_root = self.plot.get_compute_root(selection);
        let (output, is_error) = {
            match maybe_compute_root {
                Some(Ok(ComputeRootResponse {
                    root: Point { x, y },
                })) => (format!("x: {x:.4}; y: {y:.4}"), false),
                Some(Err(err)) => (err.to_string(), true),
                None => ("Loading...".to_owned(), false),
            }
        };

        let output_element = match is_error {
            false => Text::new(output),
            true => Text::new(output).style(theme::Text::Color([0.8, 0.141, 0.004].into())),
        };

        let output_row = Row::new()
            .push(Element::from("Output:"))
            .push(output_element)
            .spacing(ROW_SPACING)
            .align_items(Alignment::Center);

        Column::new()
            .push(tabs_descriptor.set_active_tab(&self.mode))
            .push(parameters_row)
            .push(output_row)
            .push(self.plot.view(selection))
            .spacing(COLUMN_SPACING)
            .width(Length::Fill)
            .into()
    }

    fn theme(&self) -> iced::Theme {
        iced::Theme::GruvboxDark
    }

    fn style(&self) -> iced::theme::Application {
        iced::theme::Application::default()
    }

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        // consider subscription as deprecated concept
        // new allows to return command which runs upon app initialization
        // here is the right place to init serial port thread
        // Only two channels are required instead of three.
        // One is created via `channel` command. The other one manually
        // Sender of manually created channel goes into app state
        // Receiver is passed via closure into serial port thread.
        // In such way we have single channel which delivers all
        // messages from serail port thread. There is no point to
        // distinguish between approximation change and response
        // messages -- they all can be handled uniformly
        let (command_sender, command_receiver) = mpsc::channel(CHANNEL_SIZE);
        let default_choice = Selection {
            mode: EquationModeRaw::SingleEquation,
            index: 0,
        };

        let mut compute_root_ui = ComputeRootUI {
            epsilon: 0.0625,
            mode: default_choice.mode,
            single_equation: SingleEquation {
                method: Method::Chord,
                equation_number: default_choice.index as u8,
            },
            system_of_equations_number: default_choice.index as u8,
            serial_port: command_sender,
            plot: FunctionPlot::new(),
        };

        compute_root_ui
            .serial_port
            .try_send(RequestPackage::FunctionPoints {
                payload: compute_root_ui.build_selection(),
            })
            .expect("Could request function's points");

        compute_root_ui
            .serial_port
            .try_send(RequestPackage::ComputeRoot {
                payload: compute_root_ui.build_compute_root_payload(),
            })
            .expect("could request root for selected equation upon start");

        (
            compute_root_ui,
            command::channel(CHANNEL_SIZE, move |sender| {
                start_loop(command_receiver, sender)
            }),
        )
    }
}
