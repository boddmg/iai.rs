#![feature(iter_arith)]
#![feature(convert)]
#![feature(alloc)]

#[macro_use]
mod utilities;  
use utilities::calculate_check_sum;
extern crate serial;
extern crate bufstream;
extern crate regex;
use regex::Regex;

use std::io;
use std::time::Duration;
use std::string::String;
use std::io::prelude::*;
use bufstream::BufStream;
use serial::prelude::*;
use serial::SystemPort;
use std::ffi::OsString; 
use std::boxed;

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

fn read_and_write_command<T: io::Read + io::Write>(port: & T,
                                                   command_to_send: &str)
                                                   -> io::Result<String> {
    let check_sum = calculate_check_sum(command_to_send.to_string());
    let mut bufPort = BufStream::new(*port);

    let mut buf = command_to_send.to_string() + int_to_upper_hex!(check_sum, 2).as_str() + "\r\n";
    try!(bufPort.write(&mut buf.into_bytes()[..]));
    bufPort.flush();

    let mut result = String::new();
    try!(bufPort.read_line(&mut result));
    Ok(result)
}

fn get_position<T: io::Read + io::Write>(port: & Box<T>) -> io::Result<f64> {
    let re = Regex::new(r"STA.(.*) ").unwrap();
    let mut statusString = read_and_write_command(port, "?99STA")
                               .unwrap_or("#99STA100000-100.000 ".to_string());
    println!("{:?}", statusString);

    for cap in re.captures_iter(statusString.as_str()) {
        println!("position:{}", cap.at(1).unwrap().to_string().to_string());
    }

    Ok(1.2)
}

fn open_serial(serial_port: &str, baud_rate: i32) -> io::Result<Box<SerialPort>>{
    let mut boxed_port = Box::new(serial::open(serial_port).unwrap());
    Ok(boxed_port)
}

#[no_mangle]
pub extern "C" fn test_function() -> f64 {
    let port = open_serial("COM7", 9600).unwrap();
    let result : Option<f64> = None;
    result = Some(get_position(& port).unwrap_or(-9999.999));
    assert_eq!(1, 2);
    match result {
        Some(expr) => expr,
        None => -9999.999,
    }
}

#[cfg(test)]
mod tests;
