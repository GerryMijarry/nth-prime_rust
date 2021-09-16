pub fn nth(n: u32) -> u32 {
        n + 1
    }


#[test]
fn test_first_prime() {
    assert_eq!(nth(0), 2);
}

#[test]
fn test_second_prime() {
    assert_eq!(nth(1), 3);
}

#[test]
fn test_sixth_prime() {
    assert_eq!(nth(5), 13);
}

#[test]
fn test_big_prime() {
    assert_eq!(nth(10_000), 104_743);
}