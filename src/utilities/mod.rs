macro_rules! int_to_upper_hex {
    ($number:expr, $length:expr) =>{{
        format!(concat!("{:0>", $length , "." , $length ,"X}"), $number)
    }}
}

pub fn calculate_check_sum(buf: String) -> i32 {
    buf.into_bytes()
       .into_iter()
       .map(|x: u8| x as i32)
       .sum::<i32>() & 0xff
}

#[cfg(test)]
mod test;
