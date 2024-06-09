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
    #[derive(Debug, Copy, Clone)]
    Direction {
        In => "in",
        Out => "out",
        InOut => "inout",
    }
}

#[derive(Clone, Debug, XmlDeserialize)]
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

#[derive(Clone, Debug, Default, XmlDeserialize)]
#[xmlserde(root = b"parameters")]
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
}

impl IntoIterator for Parameters {
    type Item = Parameter;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.parameter.into_iter()
    }
}

#[derive(Clone, Debug, XmlDeserialize)]
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
    #[xmlserde(name = b"type", ty = "child")]
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

#[derive(Clone, Debug, XmlDeserialize)]
#[xmlserde(root = b"instance-parameter")]
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
