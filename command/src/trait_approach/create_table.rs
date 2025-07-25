use super::migration::Migration;

pub struct CreateTable;

impl Migration for CreateTable {
    fn execute(&self) -> &str {
        "create table"
    }

    fn rollback(&self) -> &str {
        "drop table"
    }
}
