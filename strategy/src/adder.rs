pub struct Adder;

impl Adder {
    pub fn add<F>(x: u8, y: u8, f: F) -> u8
    where
        F: Fn(u8, u8) -> u8,
    {
        f(x, y)
    }
}

#[cfg(test)]
mod tests {
    use crate::adder::Adder;

    #[test]
    fn test_arith_adder() {
        let arith_adder = |x: u8, y: u8| x + y;

        assert_eq!(9, Adder::add(4, 5, arith_adder));
    }

    #[test]
    fn test_bool_adder() {
        let bool_adder = |x: u8, y: u8| -> u8 { if x == 1 || y == 1 { 1 } else { 0 } };

        assert_eq!(0, Adder::add(0, 0, bool_adder));
    }

    #[test]
    fn test_custom_adder() {
        let custom_adder = |x: u8, y: u8| 2 * x + y;
        assert_eq!(5, Adder::add(1, 3, custom_adder));
    }
}
