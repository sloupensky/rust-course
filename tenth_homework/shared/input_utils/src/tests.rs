#[cfg(test)]
#[test]
fn is_valid_address() {
    let address = "127.0.0.1:1111".to_string();
    let Ok(is_valid) = super::is_valid_address(&address) else {
        panic!("invalid address");
    };
    assert!(is_valid);
}

#[test]
fn not_valid_address() {
    let address = "127.0.0.1".to_string();

    match super::is_valid_address(&address) {
        Err(super::InputError::InvalidAddress(err_address)) => {
            assert_eq!(err_address, address);
        }
        _ => panic!("Expected an InvalidAddress error"),
    }
}