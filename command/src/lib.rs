pub mod add_field;
pub mod create_table;
pub mod migration;
pub mod schema;

#[cfg(test)]
mod test {
    use crate::{add_field::AddField, create_table::CreateTable, schema::Schema};

    #[test]
    fn test_schema() {
        let mut schema = Schema::new();

        let cmd = Box::new(CreateTable);
        schema.add_migration(cmd);
        let cmd = Box::new(AddField);
        schema.add_migration(cmd);

        assert_eq!(vec!["create table", "add field"], schema.execute());
        assert_eq!(vec!["remove field", "drop table"], schema.rollback());
    }
}
