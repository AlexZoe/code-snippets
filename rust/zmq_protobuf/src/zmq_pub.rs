use clap;
use protobuf::Message;
use std::{
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

mod protobuf_files;
use protobuf_files::sensor_messages;

const SLEEP_TIME_IN_MS: u64 = 1000;

pub struct SensorNode {
    context_out: Option<zmq::Context>,
    socket_out: Option<zmq::Socket>,
}

impl SensorNode {
    pub fn new(socket: &str) -> Result<Self, zmq::Error> {
        let mut node = SensorNode {
            context_out: None,
            socket_out: None,
        };

        node.context_out = Some(zmq::Context::new());
        node.socket_out = Some(node.context_out.as_ref().unwrap().socket(zmq::PUB).unwrap());
        node.socket_out.as_ref().unwrap().set_linger(200)?;
        node.socket_out.as_ref().unwrap().bind(socket)?;

        Ok(node)
    }

    pub fn broadcast(&self, sleep: u64) -> bool {
        let mut message = sensor_messages::SensorMessage::new();
        message.set_header(sensor_messages::Header::new());
        message.set_payload(sensor_messages::Payload::new());
        message.set_time_now(
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64,
        );
        message.mut_header().set_sensor_id(0);

        thread::sleep(Duration::from_millis(sleep));
        if !message.is_initialized() {
            return false;
        }

        if let Ok(bytes) = message.write_to_bytes() {
            if let Ok(_) = self.socket_out.as_ref().unwrap().send(&bytes, 0) {
                return true;
            }
        }
        false
    }
}

fn main() {
    let matches = clap::App::new("ZeroMQ Protobuf Example")
        .version("1.0")
        .author("Alexander Zoellner <example@gmail.com>")
        .about("ZeroMQ Publisher")
        .arg(
            clap::Arg::with_name("socket")
                .long("socket")
                .value_name("SOCKET")
                .help("Sets ZeroMQ socket name")
                .default_value("ipc:///tmp/sensor_rep")
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("sleep")
                .long("sleep")
                .value_name("MS")
                .help("Sleep between two messages")
                .default_value(Box::leak(SLEEP_TIME_IN_MS.to_string().into_boxed_str()))
                .takes_value(true),
        )
        .get_matches();
    if let Ok(node) = SensorNode::new(matches.value_of("socket").unwrap()) {
        loop {
            println!(
                "Broadcast status success: {}",
                node.broadcast(
                    matches
                        .value_of("sleep")
                        .unwrap()
                        .parse::<u64>()
                        .unwrap_or(SLEEP_TIME_IN_MS)
                )
            );
        }
    } else {
        println!("Call to new() failed");
    }
}
