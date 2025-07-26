use crate::formatter::Formatter;

pub struct Json;

impl Formatter for Json {
    fn format(&self, data: &crate::formatter::Data, buf: &mut String) {
        buf.push('[');
        for (k, v) in data.into_iter() {
            let entry = format!(r#"{{"{}":"{}"}}"#, k, v);
            buf.push_str(&entry);
            buf.push(',');
        }
        if !data.is_empty() {
            buf.pop();
        }
        buf.push(']');
    }
}
