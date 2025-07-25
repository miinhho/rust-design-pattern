pub trait Migration {
    fn execute(&self) -> &str;
    fn rollback(&self) -> &str;
}
