#![allow(dead_code)] // TODO (PeterlitsZo): This line need be removed in the future.

//! Defined the type [Value] and its implementation.

/// The Jisp value.
#[derive(Debug, PartialEq, Clone)]
pub(super) enum Value {
    Null,

    IFunc(u32),

    Int(i64),
    Float(f64),
    Bool(bool),
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

    /// Get the int value by giving the raw i64 value.
    pub(super) fn int(value: i64) -> Self {
        Self::Int(value)
    }

    /// Get the float value by giving the raw f64 value.
    pub(super) fn float(value: f64) -> Self {
        Self::Float(value)
    }

    /// Get the bool value by giving the raw bool value.
    pub(super) fn bool(value: bool) -> Self {
        Self::Bool(value)
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

    /// Check if the value is int and return `Some(value)` if it is.
    pub(super) fn as_int(&self) -> Option<i64> {
        match self {
            Self::Int(value) => Some(*value),
            _ => None,
        }
    }

    /// Check if the value is float and return `Some(value)` if it is.
    pub(super) fn as_float(&self) -> Option<f64> {
        match self {
            Self::Float(value) => Some(*value),
            _ => None,
        }
    }

    /// Check if the value is bool and return `Some(value)` if it is.
    pub(super) fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Bool(value) => Some(*value),
            _ => None,
        }
    }

    /// Get the kind of the value.
    pub(super) fn kind(&self) -> ValueKind {
        match self {
            Self::Null => ValueKind::Null,

            Self::IFunc(..) => ValueKind::IFunc,

            Self::Int(..) => ValueKind::Int,
            Self::Float(..) => ValueKind::Float,
            Self::Bool(..) => ValueKind::Bool,
        }
    }
}

/// The kind (type) of value.
#[derive(Debug, PartialEq, Eq)]
pub(super) enum ValueKind {
    Null,

    IFunc,

    Int,
    Float,
    Bool,
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

    #[test]
    fn test_value_as_int() {
        assert_eq!(Value::int(42).as_int(), Some(42));
    }

    #[test]
    fn test_value_as_float() {
        assert_eq!(Value::float(3.14).as_float(), Some(3.14));
    }

    #[test]
    fn test_value_as_bool() {
        assert_eq!(Value::bool(true).as_bool(), Some(true));
    }
}