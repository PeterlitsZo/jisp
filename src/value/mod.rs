#![allow(dead_code)] // TODO (PeterlitsZo): This line need be removed in the future.

//! Definded the type [Value] and its implementation.

/// The Jisp value.
#[derive(Debug, PartialEq, Clone)]
pub(super) enum Value {
    Null,

    IFunc(u32),
}

impl Value {
    /// Get the null value.
    pub(super) fn null() -> Self {
        Self::Null
    }

    /// Get the ifunc value by giving ifunc index.
    pub(super) fn ifunc(idx: u32) -> Self {
        Self::IFunc(idx)
    }

    /// Check if the value is null and return `Some(())` if it is.
    pub(super) fn as_null(&self) -> Option<()> {
        match self {
            Self::Null => Some(()),
            _ => None,
        }
    }

    /// Check if the value is ifunc and return `Some(idx)` if it is.  Here
    /// `idx` is the ifunc index in the Jisp program.
    pub(super) fn as_ifunc(&self) -> Option<u32> {
        match self {
            Self::IFunc(idx) => Some(*idx),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_as_null() {
        assert_eq!(Value::null().as_null(), Some(()));
    }

    #[test]
    fn test_value_as_ifunc() {
        assert_eq!(Value::ifunc(7).as_ifunc(), Some(7));
    }
}