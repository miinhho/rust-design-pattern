use std::str::Chars;

pub struct Interpreter<'a> {
    it: Chars<'a>,
}

impl<'a> Interpreter<'a> {
    pub fn new(infix: &'a str) -> Self {
        Self { it: infix.chars() }
    }

    fn next_char(&mut self) -> Option<char> {
        self.it.next()
    }

    pub fn interpret(&mut self, out: &mut String) {
        self.term(out);

        while let Some(op) = self.next_char() {
            if op == '+' || op == '-' {
                self.term(out);
                out.push(op);
            } else {
                panic!("Unexpected symbol '{op}'");
            }
        }
    }

    fn term(&mut self, out: &mut String) {
        match self.next_char() {
            Some(ch) if ch.is_digit(10) => out.push(ch),
            Some(ch) => panic!("Unexpected symbol '{ch}'"),
            None => panic!("Unexpected end of string"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Interpreter;

    #[test]
    fn two_plus_three() {
        let mut intr = Interpreter::new("2+3");
        let mut postfix = String::new();

        intr.interpret(&mut postfix);

        assert_eq!(postfix, "23+");
    }

    #[test]
    fn one_minus_two_plus_three_minus_four() {
        let mut intr = Interpreter::new("1-2+3-4");
        let mut postfix = String::new();

        intr.interpret(&mut postfix);

        assert_eq!(postfix, "12-3+4-");
    }
}
