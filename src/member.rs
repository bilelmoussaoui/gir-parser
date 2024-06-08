use xmlserde_derives::XmlDeserialize;

use crate::{
    attribute::Attribute,
    documentation::{DocDeprecated, DocStability, DocVersion, Documentation, SourcePosition},
    prelude::*,
    version::Version,
    Stability,
};

#[derive(Debug, XmlDeserialize)]
pub struct Member {
    #[xmlserde(name = b"name", ty = "attr")]
    name: String,
    #[xmlserde(name = b"value", ty = "attr")]
    value: String,
    #[xmlserde(name = b"c:identifier", ty = "attr")]
    c_identifier: String,
    #[xmlserde(name = b"glib:nick", ty = "attr")]
    g_nick: Option<String>,
    #[xmlserde(name = b"glib:name", ty = "attr")]
    g_name: Option<String>,
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
}

impl Member {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn c_identifier(&self) -> &str {
        &self.c_identifier
    }

    pub fn g_nick(&self) -> Option<&str> {
        self.g_nick.as_deref()
    }

    pub fn g_name(&self) -> Option<&str> {
        self.g_name.as_deref()
    }
}

impl_info!(Member);
impl_attributable!(Member);
impl_documentable!(Member);
