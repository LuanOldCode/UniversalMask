// tests/mask_tests.rs
use universal_mask::mask;

#[test]
fn test_ssn_format() {
    let ssn = "123456789";
    let masked = mask(ssn, "XXX-XX-XXXX");
    assert_eq!(masked, "123-45-6789");
}

#[test]
fn test_phone_format() {
    let phone = "1234567890";
    let masked = mask(phone, "(XXX) XXX-XXXX");
    assert_eq!(masked, "(123) 456-7890");
}

#[test]
fn test_multiple_formats() {
    let number = "123456789012";
    let masked = mask(number, "XXX-XX-XXXX | XX-XXXXXXX");
    assert_eq!(masked, "12-3456789");
}

#[test]
fn test_long_format() {
    let multi_number = "123456789012345";
    let masked = mask(multi_number, "XXX-XX-XXXX | XX-XXXXXXX | XXX-XXX-XXX-XXX-XXX");
    assert_eq!(masked, "123-456-789-012-345");
}

#[test]
fn test_empty_input() {
    let empty = "";
    let masked = mask(empty, "XXX-XX-XXXX");
    assert_eq!(masked, "");
}

#[test]
fn test_partial_format() {
    let partial = "123";
    let masked = mask(partial, "XXX-XX-XXXX");
    assert_eq!(masked, "123");
}
