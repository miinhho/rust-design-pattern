macro_rules! norm {
    ($($element: expr), *) => {
        {
            let mut n = 0.0;
            $(
                n += ($element as f64) * ($element as f64);
            )*
            n.sqrt()
        }
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_norm_mecro() {
        let x = -3 as f64;
        let y = 4 as f64;

        assert_eq!(3 as f64, norm!(x));
        assert_eq!(5 as f64, norm!(x, y));
        assert_eq!(0 as f64, norm!(0, 0, 0));
        assert_eq!(1 as f64, norm!(0.5, -0.5, 0.5, -0.5));
    }
}
