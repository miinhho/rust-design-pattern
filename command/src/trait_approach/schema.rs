use super::migration::Migration;

pub struct Schema {
    commands: Vec<Box<dyn Migration>>,
}

impl Schema {
    pub fn new() -> Self {
        Self { commands: vec![] }
    }

    pub fn add_migration(&mut self, cmd: Box<dyn Migration>) {
        self.commands.push(cmd)
    }

    pub fn execute(&self) -> Vec<&str> {
        self.commands.iter().map(|cmd| cmd.execute()).collect()
    }

    pub fn rollback(&self) -> Vec<&str> {
        self.commands
            .iter()
            .rev()
            .map(|cmd| cmd.rollback())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::trait_approach::{add_field::AddField, create_table::CreateTable, schema::Schema};

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
