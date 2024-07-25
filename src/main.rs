use function_plot::{FunctionPlot, Payload, PlotMessage, Selected};
use iced::command;
use iced::futures::channel::mpsc::{self, Sender};
use iced::futures::SinkExt;
use iced::theme::{self};
use iced::widget::pick_list;
use iced::widget::row;
use iced::widget::text_input;
use iced::widget::{button, column};
use iced::{widget::Column, Element, Settings};
use iced::{Application, Command};

use iced_aw::{tabs::Tabs, TabLabel};
use protocol::request::payloads::FunctionPointsPayload;
use protocol::request::{self, compute_method::Method, EquationModeRaw, RequestPackage};
use protocol::response::ResponsePackage;
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
    ResponseReceived(ResponsePackage),
    // do nothing when Command is exectuted purly for side effect
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
struct SingleEquiation {
    method: Method,
    equation_number: u8,
}

#[derive(Debug)]
struct ComputeRootUI {
    // this structure contains everything required to represent ui
    // and to send request
    epsilon: TNumber,
    mode: EquationModeRaw,
    single_equation: SingleEquiation,
    system_of_equtaions_number: u8,
    serial_port: Sender<RequestPackage>,
    plot: FunctionPlot,
}

impl ComputeRootUI {
    fn get_equation_number(&self) -> usize {
        let number = match self.mode {
            EquationModeRaw::SingleEquation => self.single_equation.equation_number,
            EquationModeRaw::SystemOfEquations => self.system_of_equtaions_number,
        } as usize;

        number
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
        match message {
            UIMessage::TabSelect(mode) => self.mode = mode,
            UIMessage::MethodSelect(method) => self.single_equation.method = method,
            UIMessage::Epsilon(epsilon) => self.epsilon = epsilon,
            UIMessage::SingleEquationSelect(equation_number) => {
                self.single_equation.equation_number = equation_number
            }
            UIMessage::ResponseReceived(response) => {
                let plot_message = match response {
                    ResponsePackage::InitialApproximations(response) => {
                        PlotMessage::InitialAppriximations(response)
                    }
                    ResponsePackage::ComputeRoot(response) => todo!(),
                    ResponsePackage::FunctionPoints(_) => todo!(),
                    ResponsePackage::FunctionPointsSecond(_) => todo!(),
                };
                self.plot.update(plot_message);
            }
            UIMessage::SystemOfEquationsSelect(system_number) => {
                self.system_of_equtaions_number = system_number
            }
        };

        // should not take too long to send single structure to the serail
        // port thread synchroniously
        match message {
            UIMessage::TabSelect(_)
            | UIMessage::SingleEquationSelect(_)
            | UIMessage::SystemOfEquationsSelect(_) => {
                self.serial_port
                    .try_send(RequestPackage::FunctionPoints {
                        payload: FunctionPointsPayload {
                            mode: self.mode,
                            equation_number: match self.mode {
                                EquationModeRaw::SingleEquation => {
                                    self.single_equation.equation_number
                                }
                                EquationModeRaw::SystemOfEquations => {
                                    self.system_of_equtaions_number
                                }
                            },
                        },
                    })
                    .expect("could request function points");
            }

            _ => (),
        };

        Command::none()
        // todo!();

        // command sends request and awaits response
        // Command::perform(, );
    }

    fn view(&self) -> Element<Self::Message> {
        dbg!(self);

        let single_equations = ["equiation 1", "equiation 2", "equiation 3"]
            .into_iter()
            .enumerate()
            .map(|(index, equation)| {
                let mut item = button(equation)
                    .padding(10)
                    .on_press(UIMessage::SingleEquationSelect(index as u8));

                if index != self.single_equation.equation_number as usize {
                    item = item.style(theme::Button::Secondary);
                }

                item.into()
            });

        let single_equation_tab = column!(
            Column::with_children(single_equations)
                .spacing(3.)
                .align_items(iced::Alignment::Center),
            row!(
                row!(
                    "epsilon:",
                    text_input("", &format!("{}", self.epsilon))
                        .on_input(|input| UIMessage::Epsilon(input.parse().unwrap_or(0.)))
                ),
                row!(
                    "method:",
                    pick_list(
                        [Method::Chord, Method::Secant, Method::SimpleIterationSingle],
                        Some(self.single_equation.method),
                        |method| UIMessage::MethodSelect(method)
                    )
                )
            )
        )
        .spacing(10.);

        let system_of_equations_tab = column!("equiation 1", "equiation 2")
            .spacing(3.)
            .align_items(iced::Alignment::Center);

        let selection = Selected {
            mode: self.mode,
            index: self.get_equation_number(),
        };

        Column::new()
            .push(
                Tabs::new(UIMessage::TabSelect)
                    .push(
                        EquationModeRaw::SingleEquation,
                        TabLabel::Text("Single equation".to_string()),
                        single_equation_tab,
                    )
                    .push(
                        EquationModeRaw::SystemOfEquations,
                        TabLabel::Text("System of Equations".to_string()),
                        system_of_equations_tab,
                    )
                    .set_active_tab(&self.mode),
            )
            .push(self.plot.view(selection))
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
        let (mut command_sender, command_receiver) = mpsc::channel(CHANNEL_SIZE);
        command_sender
            .try_send(RequestPackage::FunctionPoints {
                payload: FunctionPointsPayload {
                    mode: EquationModeRaw::SingleEquation,
                    equation_number: 0,
                },
            })
            .expect("Could request function's points");

        let default_choise = Selected {
            mode: EquationModeRaw::SingleEquation,
            index: 0,
        };

        (
            ComputeRootUI {
                epsilon: 0.0625,
                mode: default_choise.mode,
                single_equation: SingleEquiation {
                    method: Method::Chord,
                    equation_number: default_choise.index as u8,
                },
                system_of_equtaions_number: default_choise.index as u8,
                serial_port: command_sender,
                plot: FunctionPlot::new(),
            },
            command::channel(CHANNEL_SIZE, move |sender| {
                start_loop(command_receiver, sender)
            }),
        )
    }

    // fn subscription(&self) -> iced::Subscription<Self::Message> {
    //     // create and store sender to send commands
    //     // initialize serail port thread
    //     iced::subscription::channel(10, 10, |mut sender| async move {
    //         loop {
    //             sender.send(Message::SubscriptionInit(sender.clone()));
    //         }
    //     })
    // }
}
