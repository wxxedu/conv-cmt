use super::writer::Writer;
use std::io::Write;

pub enum CommitScope<const MAX_LEN: usize> {
    FixedLength(CommitScopeFL<MAX_LEN>),
    VariableLength(CommitScopeVL),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CommitScopeFL<const MAX_LEN: usize> {
    len: usize,
    scope: [u8; MAX_LEN],
}

impl<const MAX_LEN: usize> Default for CommitScopeFL<MAX_LEN> {
    fn default() -> Self {
        Self {
            len: 0,
            scope: [0; MAX_LEN],
        }
    }
}

impl<const MAX_LEN: usize> Write for CommitScopeFL<MAX_LEN> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        if buf.len() > MAX_LEN {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!(
                    "The length of the scope must be less than or equal to {}",
                    MAX_LEN
                ),
            ));
        }
        self.scope[..buf.len()].copy_from_slice(buf);
        self.len = buf.len();
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl<const MAX_LEN: usize> Writer for CommitScopeFL<MAX_LEN> {
    fn write_to<T: Write>(&self, target: &mut T) -> std::io::Result<()> {
        target.write_all(b"(")?;
        target.write_all(&self.scope[..self.len])?;
        target.write_all(b")")?;
        Ok(())
    }
}

impl<const MAX_LEN: usize> CommitScopeFL<MAX_LEN> {
    /// Creates a new commit scope with the given `scope`.
    pub fn new<T: AsRef<str>>(scope: T) -> std::io::Result<Self> {
        let mut res = Self::default();
        res.write(scope.as_ref().as_bytes())?;
        Ok(res)
    }

    /// Gets the length of the scope.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Gets the scope.
    pub fn scope(&self) -> &[u8] {
        &self.scope[..self.len]
    }
}

impl<const MAX_LEN: usize> std::fmt::Display for CommitScopeFL<MAX_LEN> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("(")?;
        f.write_str(std::str::from_utf8(&self.scope[..self.len]).unwrap())?;
        f.write_str(")")?;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct CommitScopeVL {
    scope: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_commit_scope() {
        let mut scope = CommitScopeFL::<10>::default();
        scope.write_all(b"feat").unwrap();
        assert_eq!(scope.len(), 4);
        assert_eq!(scope.scope(), b"feat");
        assert_eq!(scope.to_string(), "(feat)");
    }
}
