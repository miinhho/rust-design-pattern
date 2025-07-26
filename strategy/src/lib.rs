mod formatter;
mod json;
mod report;
mod text;

#[cfg(test)]
mod tests {
    use crate::json::Json;
    use crate::report::Report;
    use crate::text::Text;

    #[test]
    fn test_text() {
        let mut s = String::from("");
        Report::generate(Text, &mut s);

        assert!(s.contains("one 1"));
        assert!(s.contains("two 2"));
    }

    #[test]
    fn test_json() {
        let mut s = String::from("");
        Report::generate(Json, &mut s);

        assert!(s.contains(r#"{"one":"1"}"#));
        assert!(s.contains(r#"{"two":"2"}"#));
    }
}
