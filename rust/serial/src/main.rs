use clap::Parser;
use serialport::{Result, SerialPort, TTYPort};
use std::io::{ErrorKind::TimedOut, Read};
use std::thread;
use std::time::Duration;

// socat -d -d pty,raw,echo=0 pty,raw,echo=0

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Serial device to use for time sync
    #[arg(short, long)]
    sync_device: String,
    /// Serial device to get data from
    #[arg(short, long)]
    data_device: String,
}

fn open_serial(device: &str) -> Result<TTYPort> {
    let builder = serialport::new(device, 115200).timeout(Duration::new(2, 0));
    let mut conn = builder.open_native()?;
    let _ = conn.set_exclusive(false);
    Ok(conn)
}

fn wait_sync_signal(sync_device: &mut TTYPort) {
    let _ = sync_device.clear(serialport::ClearBuffer::Input);
    let mut buf: [u8; 5] = [0; 5];
    loop {
        match sync_device.read_exact(&mut buf) {
            Ok(_) if buf[0..3] == [3, 5, 6] => {
                println!("{:?}", buf)
            }

            Err(err) if err.kind() == TimedOut => {
                println!("Timeout");
            }

            _ => {
                println!("error, exiting");
                return;
            }
        }
    }
}

fn read_packet(data_device: &mut TTYPort) {
    let mut buf: [u8; 128] = [0; 128];
    loop {
        match data_device.read_exact(&mut buf) {
            Ok(_) => {
                println!("{:?}", buf)
            }

            Err(err) if err.kind() == TimedOut => {
                println!("Timeout");
            }

            _ => {
                println!("error, exiting");
                return;
            }
        }
    }
}

fn main() {
    let args = Args::parse();
    let mut sync_device = open_serial(&args.sync_device).expect("Cannot open timing device");
    let mut data_device = open_serial(&args.data_device).expect("Cannot open data device");

    wait_sync_signal(&mut sync_device);
    thread::sleep(Duration::from_millis(800));
    let _ = data_device.clear(serialport::ClearBuffer::Input);
    wait_sync_signal(&mut sync_device);
    read_packet(&mut data_device);
}
