use protobuf::Message;
use std::{
    env,
    thread,
    time::{
        Duration,
        SystemTime,
        UNIX_EPOCH
    }
};

mod protobuf_files;
use protobuf_files::sensor_messages;

pub struct SensorNode {
    context_in: Option<zmq::Context>,
    socket_in: Option<zmq::Socket>,
    context_out: Option<zmq::Context>,
    socket_out: Option<zmq::Socket>,
}

impl SensorNode {
    pub fn new() -> Result<Self, zmq::Error> {
        let mut node = SensorNode {
            context_in: None,
            socket_in: None,
            context_out: None,
            socket_out: None,
        };

        node.context_in = Some(zmq::Context::new());
        node.socket_in = Some(node.context_in.as_ref().unwrap()
                .socket(zmq::SUB).unwrap());
        node.socket_in.as_ref().unwrap().set_linger(200)?;
        node.socket_in.as_ref().unwrap().set_rcvtimeo(2000)?;
        node.socket_in.as_ref().unwrap().set_subscribe(b"")?;
        node.socket_in.as_ref().unwrap().connect("ipc:///tmp/sensor_req")?;

        node.context_out = Some(zmq::Context::new());
        node.socket_out = Some(node.context_out.as_ref().unwrap()
                .socket(zmq::PUB).unwrap());
        node.socket_out.as_ref().unwrap().set_linger(200)?;
        node.socket_out.as_ref().unwrap().bind("ipc:///tmp/sensor_rep")?;

        Ok(node)
    }

    pub fn broadcast(&self) -> bool {
        let mut message = sensor_messages::SensorMessage::new();
        message.set_header(sensor_messages::Header::new());
        message.set_payload(sensor_messages::Payload::new());
        message.set_time_now(SystemTime::now()
                .duration_since(UNIX_EPOCH).unwrap()
                .as_millis() as u64);
        message.mut_header().set_sensor_id(0);

        thread::sleep(Duration::from_millis(1000));
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
    if let Ok(node) = SensorNode::new() {
        println!("Broadcast status success: {}", node.broadcast());
    } else {
        println!("Call to new() failed");
    }
}
