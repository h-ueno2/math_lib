#[cfg(test)]
mod tests {
    use math::exp::*;

    #[test]
    fn squared_test() {
        assert_eq!(squared(3), 9);
    }

    #[test]
    fn cubed_test() {
        assert_eq!(cubed(2), 8);
    }

    #[test]
    fn power_test() {
        assert_eq!(power(2, 4), 16);
        assert_eq!(power(2, 10), 1024);
        assert_eq!(power(2, 24), 16777216);
    }
}
