use std::error::Error;
use std::io::{self};
use std::time::Duration;

use iced::futures::channel::mpsc::{Receiver, Sender};

use iced::futures::SinkExt;
use protocol::response::{ComputeRootResponse, MethodError, ResponsePackage};
use protocol::{byte_serializable::ByteSerializable, request::RequestPackage};
use protocol::{is_signature_valid, PROTOCOL_SIGNATURE, PROTOCOL_SIGNATURE_SIZE};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio_serial::{ClearBuffer, SerialPort, SerialPortBuilderExt, SerialStream};

use crate::UIMessage;

// connect
// verify signature
// enter main loop

pub async fn start_loop(packages: Receiver<RequestPackage>, messages: Sender<UIMessage>) {
    let serial_port = tokio_serial::new("/dev/ttyACM0", 250_000)
        .open_native_async()
        .expect("succesfully open");

    let serial_port = verify_signature(serial_port)
        .await
        .expect("Signature valid");
    println!("signature verified");

    let read_buffer_long: [u8; protocol::LONG_PACKAGE_SIZE] = [0; protocol::LONG_PACKAGE_SIZE];
    let read_buffer_short: [u8; protocol::PACKAGE_SIZE] = [0; protocol::PACKAGE_SIZE];

    let mut context = LoopContext {
        packages,
        serial_port,
        read_buffer_long,
        read_buffer_short,
        messages,
    };

    loop {
        match loop_iteration(&mut context).await {
            Ok(_) => continue,
            Err(err) => eprintln!("{}", err),
        }
    }
}

struct LoopContext {
    messages: Sender<UIMessage>,
    packages: Receiver<RequestPackage>,
    serial_port: SerialStream,
    read_buffer_long: [u8; protocol::LONG_PACKAGE_SIZE],
    read_buffer_short: [u8; protocol::PACKAGE_SIZE],
}

async fn loop_iteration(
    LoopContext {
        packages,
        serial_port,
        read_buffer_long,
        read_buffer_short,
        messages,
    }: &mut LoopContext,
) -> Result<(), Box<dyn Error>> {
    // check for available commands
    // if some -> convert to requests
    // if none -> request approximations

    // try_next,'cause no need to wait for new package
    // if we have nothing to send, send InitialApproximations request
    let command = packages.try_next();
    let request = if let Ok(Some(value)) = command {
        value
    } else {
        RequestPackage::InitialApproximations
    };

    AsyncWriteExt::write(serial_port, &request.to_bytes()).await?;
    // wait for port to become readable
    serial_port.readable().await?;
    let read_buffer: &mut [u8] = match request {
        RequestPackage::FunctionPoints { .. } => read_buffer_long,
        RequestPackage::InitialApproximations => read_buffer_short,
        RequestPackage::ComputeRoot { .. } => read_buffer_short,
    };
    // read and parse data
    serial_port.read_exact(read_buffer).await?;
    let response: protocol::response::ResponsePackage = match &request {
        RequestPackage::FunctionPoints { payload } => match payload.mode {
            protocol::request::EquationModeRaw::SingleEquation => {
                protocol::response::FunctionPointsResponse::from(&*read_buffer_long).into()
            }
            protocol::request::EquationModeRaw::SystemOfEquations => {
                messages
                    .send(UIMessage::ResponseReceived(
                        request.clone(),
                        protocol::response::FunctionPointsResponse::from(&*read_buffer_long).into(),
                    ))
                    .await?;
                serial_port.read_exact(read_buffer_long).await?;
                protocol::response::ResponsePackage::FunctionPointsSecond(
                    protocol::response::FunctionPointsResponse::from(&*read_buffer_long),
                )
            }
        },
        RequestPackage::InitialApproximations => {
            protocol::response::InitialApproximationsResponse::from_bytes(read_buffer_short).into()
        }
        RequestPackage::ComputeRoot { .. } => {
            Result::<ComputeRootResponse, MethodError>::from_bytes(read_buffer_short).into()
        }
    };

    if let ResponsePackage::ComputeRoot(r) = response {
        let _ = dbg!(r);
    }

    messages
        .send(UIMessage::ResponseReceived(request, response))
        .await?;

    // prevent message flood
    tokio::time::sleep(Duration::from_millis(500)).await;

    Ok(())
}

async fn verify_signature(mut serial_port: SerialStream) -> io::Result<SerialStream> {
    // if signature was not received within first 16 bytes, then connection
    // could not be insteblished
    let mut buffer = [0; PROTOCOL_SIGNATURE_SIZE * 2];

    serial_port.read_exact(&mut buffer).await?;
    if !is_signature_valid(&buffer) {
        return Err(io::Error::new(
            io::ErrorKind::ConnectionRefused,
            "Signature is not valid. Is this device sertified by White Horizont corporation?",
        ));
    }
    serial_port
        .write_all(&PROTOCOL_SIGNATURE.to_le_bytes())
        .await?;
    serial_port.flush().await?;
    // clear leftover data
    serial_port.clear(ClearBuffer::Input)?;

    Ok(serial_port)
}

// command has an async function which sends state to this thread
// and awaits response back

// here public async function will be
// they expose api to communicate to this thread
