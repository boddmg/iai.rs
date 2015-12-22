extern crate serial;

use std::io;
use std::time::Duration;
use std::string::String;

use std::io::prelude::*;
use serial::prelude::*;

fn get_baudrate_enum(baud_rate: i32) -> serial::BaudRate {
    match baud_rate {
        9600 => serial::Baud9600,
        38400 => serial::Baud38400,
        115200 => serial::Baud115200,
        _ => serial::Baud9600,
    }
}

fn setup_port<T: SerialPort>(port: &mut T, baud_rate: i32) -> io::Result<()> {
    try!(port.reconfigure(&|settings| {
        try!(settings.set_baud_rate(get_baudrate_enum(baud_rate)));
        settings.set_char_size(serial::Bits8);
        settings.set_parity(serial::ParityNone);
        settings.set_stop_bits(serial::Stop1);
        settings.set_flow_control(serial::FlowNone);
        Ok(())
    }));

    try!(port.set_timeout(Duration::from_millis(5000)));
    Ok(())
}

fn read_and_write_command(port: &mut SerialPort, command_to_send: String) -> io::Result<String> {
    let mut buf = command_to_send.into_bytes();

    try!(port.write(&mut buf[..]));
    try!(port.read(&mut buf[..]));
    let read_string = String::from_utf8(buf);
    println!("{:?}", read_string);
    Ok(read_string.unwrap())
}

fn int_to_upper_hex(number: i32) -> String {
    format!("{:X}", number)
}

fn get_position(port: &mut SerialPort) -> io::Result<f64> {
    let mut buf = "?[99]STA".to_string().into_bytes();
    try!(port.write(&mut buf));
    Ok(1.2)
}

extern crate libc;
use std::ffi::CStr;

#[no_mangle]
pub extern "C" fn test_function() -> String {
    let arg = "COM1";
    let mut port = serial::open(&arg).unwrap();
    setup_port(&mut port, 9600).unwrap();
    "abc".to_string()
}

fn main() {
    test_function();
}
