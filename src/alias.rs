use xmlserde_derives::XmlDeserialize;

use crate::{
    attribute::Attribute,
    documentation::{DocDeprecated, DocStability, DocVersion, Documentation, SourcePosition},
    prelude::*,
    r#type::AnyType,
    version::Version,
    Stability,
};

#[derive(Clone, Debug, XmlDeserialize)]
#[xmlserde(root = b"alias")]
#[xmlserde(deny_unknown_fields)]
pub struct Alias {
    #[xmlserde(name = b"name", ty = "attr")]
    name: String,
    #[xmlserde(name = b"c:type", ty = "attr")]
    c_type: String,
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
    type_: Option<AnyType>,
}

impl Alias {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn c_type(&self) -> &str {
        &self.c_type
    }

    pub fn ty(&self) -> Option<&AnyType> {
        self.type_.as_ref()
    }
}

impl_info!(Alias);
impl_attributable!(Alias);
impl_documentable!(Alias);
