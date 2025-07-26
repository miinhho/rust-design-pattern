use crate::formatter::Formatter;

pub struct Text;

impl Formatter for Text {
    fn format(&self, data: &crate::formatter::Data, buf: &mut String) {
        for (k, v) in data {
            let entry = format!("{k} {v}\n");
            buf.push_str(&entry);
        }
    }
}
