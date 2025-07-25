use crate::migration::Migration;

pub struct AddField;

impl Migration for AddField {
    fn execute(&self) -> &str {
        "add field"
    }

    fn rollback(&self) -> &str {
        "remove field"
    }
}
