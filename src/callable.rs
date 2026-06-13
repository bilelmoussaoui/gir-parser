use xmlserde_derives::XmlDeserialize;

use crate::{
    function::Function,
    method::Method,
    parameter::Parameters,
    return_value::ReturnValue,
    traits::{Callable as CallableTrait, FunctionLike},
};

#[derive(Clone, Debug, PartialEq, Eq, XmlDeserialize)]
pub enum Callable {
    #[xmlserde(name = b"constructor")]
    Constructor(Function),
    #[xmlserde(name = b"method")]
    Method(Method),
    #[xmlserde(name = b"function")]
    Function(Function),
}

impl Callable {
    pub fn is_constructor(&self) -> bool {
        matches!(self, Self::Constructor(_))
    }

    pub fn is_method(&self) -> bool {
        matches!(self, Self::Method(_))
    }

    pub fn is_function(&self) -> bool {
        matches!(self, Self::Function(_))
    }

    pub fn as_function(&self) -> Option<&Function> {
        match self {
            Self::Constructor(f) | Self::Function(f) => Some(f),
            _ => None,
        }
    }

    pub fn as_method(&self) -> Option<&Method> {
        match self {
            Self::Method(m) => Some(m),
            _ => None,
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Self::Constructor(f) | Self::Function(f) => f.name(),
            Self::Method(m) => m.name(),
        }
    }

    pub fn c_identifier(&self) -> Option<&str> {
        match self {
            Self::Constructor(f) | Self::Function(f) => f.c_identifier(),
            Self::Method(m) => m.c_identifier(),
        }
    }

    pub fn moved_to(&self) -> Option<&str> {
        match self {
            Self::Constructor(f) | Self::Function(f) => f.moved_to(),
            Self::Method(m) => m.moved_to(),
        }
    }

    pub fn throws(&self) -> bool {
        match self {
            Self::Constructor(f) | Self::Function(f) => f.throws(),
            Self::Method(m) => m.throws(),
        }
    }

    pub fn return_value(&self) -> &ReturnValue {
        match self {
            Self::Constructor(f) | Self::Function(f) => f.return_value(),
            Self::Method(m) => m.return_value(),
        }
    }

    pub fn parameters(&self) -> &Parameters {
        match self {
            Self::Constructor(f) | Self::Function(f) => f.parameters(),
            Self::Method(m) => m.parameters(),
        }
    }
}
