use xmlserde::xml_serde_enum;
use xmlserde_derives::XmlDeserialize;

use crate::{
    array::Array,
    attribute::Attribute,
    documentation::{DocDeprecated, DocStability, DocVersion, Documentation, SourcePosition},
    prelude::*,
    r#type::Type,
    FunctionScope, TransferOwnership,
};

xml_serde_enum! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    Direction {
        In => "in",
        Out => "out",
        InOut => "inout",
    }
}

#[derive(Clone, Debug, PartialEq, Eq, XmlDeserialize)]
pub enum ParameterType {
    #[xmlserde(name = b"type")]
    Type(Type),
    #[xmlserde(name = b"array")]
    Array(Array),
    #[xmlserde(name = b"varargs")]
    VarArgs,
}

impl From<crate::r#type::Type> for ParameterType {
    fn from(value: crate::r#type::Type) -> Self {
        Self::Type(value)
    }
}

impl From<crate::r#type::AnyType> for ParameterType {
    fn from(value: crate::r#type::AnyType) -> Self {
        match value {
            crate::r#type::AnyType::Array(arr) => Self::Array(arr),
            crate::r#type::AnyType::Type(ty) => Self::Type(ty),
        }
    }
}

/// Represents either an instance parameter or a regular parameter
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AnyParameter<'a> {
    Instance(&'a InstanceParameter),
    Regular(&'a Parameter),
}

impl<'a> AnyParameter<'a> {
    pub fn name(&self) -> &str {
        match self {
            Self::Instance(p) => p.name(),
            Self::Regular(p) => p.name(),
        }
    }

    pub fn is_instance(&self) -> bool {
        matches!(self, Self::Instance(_))
    }

    pub fn is_regular(&self) -> bool {
        matches!(self, Self::Regular(_))
    }

    pub fn as_instance(&self) -> &'a InstanceParameter {
        match self {
            Self::Instance(p) => p,
            _ => unreachable!(),
        }
    }

    pub fn as_regular(&self) -> &'a Parameter {
        match self {
            Self::Regular(p) => p,
            _ => unreachable!(),
        }
    }

    pub fn is_nullable(&self) -> Option<bool> {
        match self {
            Self::Instance(p) => p.is_nullable(),
            Self::Regular(p) => p.is_nullable(),
        }
    }

    pub fn is_allow_none(&self) -> Option<bool> {
        match self {
            Self::Instance(p) => p.is_allow_none(),
            Self::Regular(p) => p.is_allow_none(),
        }
    }

    pub fn direction(&self) -> Option<Direction> {
        match self {
            Self::Instance(p) => p.direction(),
            Self::Regular(p) => p.direction(),
        }
    }

    pub fn is_caller_allocates(&self) -> Option<bool> {
        match self {
            Self::Instance(p) => p.is_caller_allocates(),
            Self::Regular(p) => p.is_caller_allocates(),
        }
    }

    pub fn transfer_ownership(&self) -> Option<TransferOwnership> {
        match self {
            Self::Instance(p) => p.transfer_ownership(),
            Self::Regular(p) => p.transfer_ownership(),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq, XmlDeserialize)]
#[xmlserde(root = b"parameters")]
#[xmlserde(deny_unknown_fields)]
pub struct Parameters {
    #[xmlserde(name = b"instance-parameter", ty = "child")]
    instance_parameter: Option<InstanceParameter>,
    #[xmlserde(name = b"parameter", ty = "child")]
    parameter: Vec<Parameter>,
}

impl Parameters {
    pub fn is_empty(&self) -> bool {
        self.instance_parameter.is_none() && self.parameter.is_empty()
    }

    pub fn instance(&self) -> Option<&InstanceParameter> {
        self.instance_parameter.as_ref()
    }

    pub fn inner(&self) -> &[Parameter] {
        &self.parameter
    }

    /// Returns an iterator over all parameters
    pub fn all(&self) -> impl Iterator<Item = AnyParameter<'_>> {
        self.instance_parameter
            .iter()
            .map(AnyParameter::Instance)
            .chain(self.parameter.iter().map(AnyParameter::Regular))
    }
}

impl IntoIterator for Parameters {
    type Item = Parameter;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.parameter.into_iter()
    }
}

impl<'a> IntoIterator for &'a Parameters {
    type Item = &'a Parameter;
    type IntoIter = std::slice::Iter<'a, Parameter>;

    fn into_iter(self) -> Self::IntoIter {
        self.parameter.iter()
    }
}

impl AsRef<[Parameter]> for Parameters {
    fn as_ref(&self) -> &[Parameter] {
        &self.parameter
    }
}

impl std::ops::Deref for Parameters {
    type Target = [Parameter];

    fn deref(&self) -> &Self::Target {
        &self.parameter
    }
}

impl std::ops::Index<usize> for Parameters {
    type Output = Parameter;

    fn index(&self, index: usize) -> &Self::Output {
        &self.parameter[index]
    }
}

#[derive(Clone, Debug, PartialEq, Eq, XmlDeserialize)]
#[xmlserde(root = b"parameter")]
#[xmlserde(deny_unknown_fields)]
pub struct Parameter {
    #[xmlserde(name = b"name", ty = "attr")]
    name: String,
    #[xmlserde(name = b"transfer-ownership", ty = "attr")]
    transfer: Option<TransferOwnership>,
    #[xmlserde(name = b"nullable", ty = "attr")]
    nullable: Option<bool>,
    #[xmlserde(name = b"allow-none", ty = "attr")]
    allow_none: Option<bool>,
    #[xmlserde(name = b"introspectable", ty = "attr")]
    introspectable: Option<bool>,
    #[xmlserde(name = b"scope", ty = "attr")]
    scope: Option<FunctionScope>,
    #[xmlserde(name = b"closure", ty = "attr")]
    closure: Option<usize>,
    #[xmlserde(name = b"destroy", ty = "attr")]
    destroy: Option<usize>,
    #[xmlserde(name = b"direction", ty = "attr")]
    direction: Option<Direction>,
    #[xmlserde(name = b"caller-allocates", ty = "attr")]
    caller_allocates: Option<bool>,
    #[xmlserde(name = b"optional", ty = "attr")]
    optional: Option<bool>,
    #[xmlserde(name = b"skip", ty = "attr")]
    skip: Option<bool>,
    // Documentation
    #[xmlserde(name = b"doc", ty = "child")]
    doc: Option<Documentation>,
    #[xmlserde(name = b"doc-deprecated", ty = "child")]
    doc_deprecated: Option<DocDeprecated>,
    #[xmlserde(name = b"doc-stability", ty = "child")]
    doc_stability: Option<DocStability>,
    #[xmlserde(name = b"doc-version", ty = "child")]
    doc_version: Option<DocVersion>,
    #[xmlserde(name = b"source-position", ty = "child")]
    source_position: Option<SourcePosition>,
    // Attributes: 0 or more
    #[xmlserde(name = b"attribute", ty = "child")]
    attributes: Vec<Attribute>,
    #[xmlserde(ty = "untag")]
    type_: Option<ParameterType>,
}

impl Parameter {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_nullable(&self) -> Option<bool> {
        self.nullable
    }

    pub fn is_allow_none(&self) -> Option<bool> {
        self.allow_none
    }

    pub fn is_introspectable(&self) -> bool {
        self.introspectable.unwrap_or(true)
    }

    pub fn scope(&self) -> Option<FunctionScope> {
        self.scope
    }

    pub fn closure(&self) -> Option<usize> {
        self.closure
    }

    pub fn destroy(&self) -> Option<usize> {
        self.destroy
    }

    pub fn direction(&self) -> Option<Direction> {
        self.direction
    }

    pub fn is_caller_allocates(&self) -> Option<bool> {
        self.caller_allocates
    }

    pub fn is_optional(&self) -> Option<bool> {
        self.optional
    }

    pub fn is_skip(&self) -> Option<bool> {
        self.skip
    }

    pub fn transfer_ownership(&self) -> Option<TransferOwnership> {
        self.transfer
    }

    pub fn ty(&self) -> Option<&ParameterType> {
        self.type_.as_ref()
    }
}

impl_attributable!(Parameter);
impl_documentable!(Parameter);

#[derive(Clone, Debug, PartialEq, Eq, XmlDeserialize)]
#[xmlserde(root = b"instance-parameter")]
#[xmlserde(deny_unknown_fields)]
pub struct InstanceParameter {
    #[xmlserde(name = b"name", ty = "attr")]
    name: String,
    #[xmlserde(name = b"transfer-ownership", ty = "attr")]
    transfer: Option<TransferOwnership>,
    #[xmlserde(name = b"nullable", ty = "attr")]
    nullable: Option<bool>,
    #[xmlserde(name = b"allow-none", ty = "attr")]
    allow_none: Option<bool>,
    #[xmlserde(name = b"direction", ty = "attr")]
    direction: Option<Direction>,
    #[xmlserde(name = b"caller-allocates", ty = "attr")]
    caller_allocates: Option<bool>,
    // Documentation
    #[xmlserde(name = b"doc", ty = "child")]
    doc: Option<Documentation>,
    #[xmlserde(name = b"doc-deprecated", ty = "child")]
    doc_deprecated: Option<DocDeprecated>,
    #[xmlserde(name = b"doc-stability", ty = "child")]
    doc_stability: Option<DocStability>,
    #[xmlserde(name = b"doc-version", ty = "child")]
    doc_version: Option<DocVersion>,
    #[xmlserde(name = b"source-position", ty = "child")]
    source_position: Option<SourcePosition>,
    #[xmlserde(name = b"type", ty = "child")]
    type_: Option<Type>,
}

impl InstanceParameter {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_nullable(&self) -> Option<bool> {
        self.nullable
    }

    pub fn is_allow_none(&self) -> Option<bool> {
        self.allow_none
    }

    pub fn direction(&self) -> Option<Direction> {
        self.direction
    }

    pub fn is_caller_allocates(&self) -> Option<bool> {
        self.caller_allocates
    }

    pub fn transfer_ownership(&self) -> Option<TransferOwnership> {
        self.transfer
    }

    pub fn ty(&self) -> Option<&Type> {
        self.type_.as_ref()
    }
}

impl_documentable!(InstanceParameter);
