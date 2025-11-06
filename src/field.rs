use xmlserde_derives::XmlDeserialize;

use crate::{
    array::Array,
    attribute::Attribute,
    callback::Callback,
    documentation::{DocDeprecated, DocStability, DocVersion, Documentation, SourcePosition},
    prelude::*,
    r#type::Type,
    version::Version,
    Stability,
};

#[derive(Clone, Debug, PartialEq, Eq, XmlDeserialize)]
#[allow(clippy::large_enum_variant)]
pub enum FieldType {
    #[xmlserde(name = b"type")]
    Type(Type),
    #[xmlserde(name = b"callback")]
    Callback(Callback),
    #[xmlserde(name = b"array")]
    Array(Array),
}

impl FieldType {
    pub fn is_type(&self) -> bool {
        matches!(self, Self::Type(_))
    }

    pub fn as_type(&self) -> &Type {
        if let Self::Type(t) = &self {
            t
        } else {
            unreachable!()
        }
    }

    pub fn is_callback(&self) -> bool {
        matches!(self, Self::Callback(_))
    }

    pub fn as_callback(&self) -> &Callback {
        if let Self::Callback(c) = &self {
            c
        } else {
            unreachable!()
        }
    }

    pub fn is_array(&self) -> bool {
        matches!(self, Self::Array(_))
    }

    pub fn as_array(&self) -> &Array {
        if let Self::Array(a) = &self {
            a
        } else {
            unreachable!()
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, XmlDeserialize)]
#[xmlserde(root = b"field")]
#[xmlserde(deny_unknown_fields)]
pub struct Field {
    #[xmlserde(name = b"name", ty = "attr")]
    name: String,
    #[xmlserde(name = b"readable", ty = "attr")]
    readable: Option<bool>,
    #[xmlserde(name = b"writable", ty = "attr")]
    writable: Option<bool>,
    // Seems to be set by libgee, which is a weird use case anyways
    // Kept hidden from the external API as it is not supposed to be set per the spec
    #[xmlserde(name = b"nullable", ty = "attr")]
    _nullable: Option<bool>,
    #[xmlserde(name = b"private", ty = "attr")]
    private: Option<bool>,
    #[xmlserde(name = b"bits", ty = "attr")]
    bits: Option<u8>,
    // Common attributes
    #[xmlserde(name = b"introspectable", ty = "attr")]
    introspectable: Option<bool>,
    #[xmlserde(name = b"deprecated", ty = "attr")]
    deprecated: Option<bool>,
    #[xmlserde(name = b"version", ty = "attr")]
    version: Option<Version>,
    #[xmlserde(name = b"deprecated-version", ty = "attr")]
    deprecated_version: Option<Version>,
    #[xmlserde(name = b"stability", ty = "attr")]
    stability: Option<Stability>,
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
    type_: FieldType,
}

impl Field {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_readable(&self) -> bool {
        self.readable.unwrap_or(false)
    }

    pub fn is_writable(&self) -> bool {
        self.writable.unwrap_or(false)
    }

    pub fn is_private(&self) -> bool {
        self.private.unwrap_or(false)
    }

    pub fn bits(&self) -> Option<u8> {
        self.bits
    }

    pub fn ty(&self) -> &FieldType {
        &self.type_
    }
}

impl_info!(Field);
impl_documentable!(Field);
impl_attributable!(Field);
