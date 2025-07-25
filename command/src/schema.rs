use crate::migration::Migration;

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
