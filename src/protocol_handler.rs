use core::mem::size_of;
use protocol::byte_serializable::ByteSerializable;
use protocol::point::Point;
use protocol::point::PointCoordinate;
use protocol::request::payloads::ComputeRootPayload;
use protocol::request::RequestPackage;
use protocol::response::ComputeRootResponse;
use protocol::response::InitialApproximationsResponse;
use protocol::response::MethodError;
use protocol::TNumber;
use protocol::PACKAGE_SIZE;

use ruduino::{cores::current::USART0, modules::HardwareUsart};

use crate::equations::Equations;
use crate::usart::Usart;

type PointsHandler<'a> = &'a mut dyn FnMut(
    &mut dyn FnMut(TNumber) -> (TNumber, PointCoordinate),
    &mut dyn FnMut(Point) -> (),
) -> ();
type InitialApproximationHandler<'b> = &'b mut dyn FnMut() -> InitialApproximationsResponse;
type ComputeRootHandler<'c> = &'c mut dyn FnMut(ComputeRootPayload) -> Result<Point, MethodError>;

pub struct Connection<'aa, 'a, 'b, 'c, T: HardwareUsart> {
    channel: &'aa Usart<T>,
    function_points_handler: Option<PointsHandler<'a>>,
    function_initial_approximation: Option<InitialApproximationHandler<'b>>,
    function_compute_root: Option<ComputeRootHandler<'c>>,
    context: Equations,
}

impl<'aa, 'a, 'b, 'c> Connection<'aa, 'a, 'b, 'c, USART0> {
    // send protocol signature
    // when correct protocol singature is echoed back
    // await for requests
    pub fn new(
        channel: &'aa Usart<USART0>,
        context: Equations,
    ) -> Connection<'aa, 'a, 'b, 'c, USART0> {
        // for some reason when arduino is first plugged in
        // it sends 0xfe, 0xfd or 0xff byte before the protocol signature.
        // Noticable, that if only two bytes are sent at a time, no additional bytes
        // are sent.
        // Send signature twice. This mitigates effect of up to 8 unexpected bytes.
        channel.write_blocking(&protocol::PROTOCOL_SIGNATURE.to_le_bytes());
        channel.write_blocking(&protocol::PROTOCOL_SIGNATURE.to_le_bytes());

        let mut received = [0_u8; size_of::<u64>()];
        loop {
            channel.read_blocking(&mut received);

            // TODO: replace with `is_signature_valid` from
            // protocol's crate
            let received = u64::from_le_bytes(received);
            // blink(5, 100);
            if received == protocol::PROTOCOL_SIGNATURE {
                break;
            }
        }

        Connection {
            channel,
            function_points_handler: None,
            function_initial_approximation: None,
            function_compute_root: None,
            context,
        }
    }

    pub fn handle_request(&mut self) {
        let mut incoming_data = [0_u8; PACKAGE_SIZE];
        self.channel.read_blocking(&mut incoming_data);

        let request = RequestPackage::from_bytes(&incoming_data);
        match request {
            RequestPackage::FunctionPoints { payload } => {
                if let Some(handler) = &mut self.function_points_handler {
                    let mut writer = |point: Point| self.channel.write_blocking(&point.to_bytes());
                    match payload.mode {
                        protocol::request::EquationModeRaw::SingleEquation => {
                            let equation = &self.context.single[payload.equation_number as usize];
                            handler(
                                &mut |x| ((equation.function)(x), PointCoordinate::Y),
                                &mut writer,
                            );
                        }
                        protocol::request::EquationModeRaw::SystemOfEquations => {
                            let mut system =
                                self.context.systems[payload.equation_number as usize].clone();
                            handler(&mut system.first.function, &mut writer);
                            handler(&mut system.second.function, &mut writer);
                        }
                    };
                }
            }
            RequestPackage::InitialApproximations => {
                if let Some(handler) = &mut self.function_initial_approximation {
                    let bytes = handler().to_bytes();
                    self.channel.write_blocking(&bytes);
                }
            }
            RequestPackage::ComputeRoot { payload } => {
                if let Some(handler) = &mut self.function_compute_root {
                    let bytes = handler(payload)
                        .map(|point| ComputeRootResponse { root: point })
                        .to_bytes();
                    self.channel.write_blocking(&bytes);
                }
            }
        }
    }

    pub fn set_points_handler(&mut self, handler: PointsHandler<'a>) {
        self.function_points_handler = Some(handler);
    }
    pub fn set_initial_approximation(&mut self, handler: InitialApproximationHandler<'b>) {
        self.function_initial_approximation = Some(handler);
    }
    pub fn set_compute_root(&mut self, handler: ComputeRootHandler<'c>) {
        self.function_compute_root = Some(handler);
    }
}
