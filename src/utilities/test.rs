#[macro_use]
use super::*;

#[test]
fn test_int_to_upper_hex() {
	let number = 0x34;
    assert!(int_to_upper_hex!(0x34, 2) == "34");
    assert!(int_to_upper_hex!(0x12, 2) == "12");
    assert!(int_to_upper_hex!(0x12, 4) == "0012");
}

#[test]
fn test_calculate_check_sum(){
	// let sum = calculate_check_sum("#99234".to_string());
	assert!(calculate_check_sum("#99234".to_string()) == 0x2e);
	assert!(calculate_check_sum("!99PSE   5 30.30 100  100.000  200.000".to_string()) == 0xf4);
}

#[test]
fn it_work() {
    // assert!(test_function() == "abc".to_string());
}
