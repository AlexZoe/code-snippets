use clap;
use protobuf::Message;

mod protobuf_files;
use protobuf_files::sensor_messages;

pub struct SensorNode {
    context_in: Option<zmq::Context>,
    socket_in: Option<zmq::Socket>,
}

impl SensorNode {
    pub fn new(socket: &str) -> Result<Self, zmq::Error> {
        let mut node = SensorNode {
            context_in: None,
            socket_in: None,
        };

        node.context_in = Some(zmq::Context::new());
        node.socket_in = Some(node.context_in.as_ref().unwrap().socket(zmq::SUB).unwrap());
        node.socket_in.as_ref().unwrap().set_linger(200)?;
        node.socket_in.as_ref().unwrap().set_rcvtimeo(2000)?;
        // Don't filter out any messages, i.e. subscribe to everything
        node.socket_in.as_ref().unwrap().set_subscribe(b"")?;
        node.socket_in.as_ref().unwrap().connect(socket)?;

        Ok(node)
    }

    pub fn listen(&self) -> bool {
        let mut message = sensor_messages::SensorMessage::new();
        message.set_header(sensor_messages::Header::new());
        message.set_payload(sensor_messages::Payload::new());

        let msg = self.socket_in.as_ref().unwrap().recv_msg(0);
        if let Ok(msg) = msg {
            println!(
                "Message is: {:#?}",
                sensor_messages::SensorMessage::parse_from_bytes(&msg.to_vec())
            );
            return true;
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
        .get_matches();
    if let Ok(node) = SensorNode::new(matches.value_of("socket").unwrap()) {
        loop {
            println!("Parsed message: {}", node.listen());
        }
    } else {
        println!("Call to new() failed");
    }
}
