mod parsers;

use serial;
use structopt;
use structopt_derive::StructOpt;
use xmodem::{Xmodem, Progress};

use std::path::PathBuf;
use std::time::Duration;

use structopt::StructOpt;
use serial::core::{CharSize, BaudRate, StopBits, FlowControl, SerialDevice, SerialPortSettings};

use parsers::{parse_width, parse_stop_bits, parse_flow_control, parse_baud_rate};

#[derive(StructOpt, Debug)]
#[structopt(about = "Write to TTY using the XMODEM protocol by default.")]
struct Opt {
    #[structopt(short = "i", help = "Input file (defaults to stdin if not set)", parse(from_os_str))]
    input: Option<PathBuf>,

    #[structopt(short = "b", long = "baud", parse(try_from_str = "parse_baud_rate"),
                help = "Set baud rate", default_value = "115200")]
    baud_rate: BaudRate,

    #[structopt(short = "t", long = "timeout", parse(try_from_str),
                help = "Set timeout in seconds", default_value = "10")]
    timeout: u64,

    #[structopt(short = "w", long = "width", parse(try_from_str = "parse_width"),
                help = "Set data character width in bits", default_value = "8")]
    char_width: CharSize,

    #[structopt(help = "Path to TTY device", parse(from_os_str))]
    tty_path: PathBuf,

    #[structopt(short = "f", long = "flow-control", parse(try_from_str = "parse_flow_control"),
                help = "Enable flow control ('hardware' or 'software')", default_value = "none")]
    flow_control: FlowControl,

    #[structopt(short = "s", long = "stop-bits", parse(try_from_str = "parse_stop_bits"),
                help = "Set number of stop bits", default_value = "1")]
    stop_bits: StopBits,

    #[structopt(short = "r", long = "raw", help = "Disable XMODEM")]
    raw: bool,
}

fn main() {
    use std::fs::File;
    use std::io::{self, BufReader};

    let opt = Opt::from_args();
    let mut port = serial::open(&opt.tty_path).expect("path points to invalid TTY");

    // FIXME: Implement the `ttywrite` utility.
    if let Ok(mut port_set) = port.read_settings() {
        port_set.set_baud_rate(opt.baud_rate).unwrap();
        port_set.set_char_size(opt.char_width);
        port_set.set_stop_bits(opt.stop_bits);
        port_set.set_flow_control(opt.flow_control);
        port.write_settings(&port_set).unwrap();
    }
    port.set_timeout(Duration::new(opt.timeout, 0)).unwrap();

    let mut _file:Box<dyn io::Read> = if let Some(path) = opt.input {
        Box::new(File::open(path).unwrap())
    } else {
        Box::new(io::stdin())
    };

    if opt.raw {
        io::copy(&mut _file, &mut port).unwrap();
    } else {
        Xmodem::transmit_with_progress(_file, port, progress_fn).unwrap();
    }
}

fn progress_fn(progress: Progress) {
    println!("Progress: {:?}", progress);
}