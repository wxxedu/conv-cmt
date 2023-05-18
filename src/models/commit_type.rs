use std::{fmt::Display, io::Write};

use super::writer::Writer;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CommitType<'a> {
    name: &'a str,
    description: &'a str,
}

impl<'a> CommitType<'a> {
    pub fn new(name: &'a str, description: &'a str) -> Self {
        Self { name, description }
    }

    pub fn name(&self) -> &'a str {
        self.name
    }

    pub fn description(&self) -> &'a str {
        self.description
    }
}

impl<'a> Writer for CommitType<'a> {
    fn write_to<T: Write>(&self, target: &mut T) -> std::io::Result<()> {
        target.write_all(self.name.as_bytes())?;
        target.write_all(b": ")?;
        target.write_all(self.description.as_bytes())?;
        Ok(())
    }
}

impl<'a> Display for CommitType<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name)?;
        f.write_str(": ")?;
        f.write_str(self.description)?;
        Ok(())
    }
}
