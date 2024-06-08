use xmlserde_derives::XmlDeserialize;

use crate::{
    attribute::Attribute,
    documentation::{DocDeprecated, DocStability, DocVersion, Documentation, SourcePosition},
    parameter::Parameters,
    prelude::*,
    version::Version,
    Stability,
};

#[derive(Debug, XmlDeserialize)]
pub struct FunctionMacro {
    // Callable attributes
    #[xmlserde(name = b"name", ty = "attr")]
    name: String,
    #[xmlserde(name = b"c:identifier", ty = "attr")]
    c_identifier: Option<String>,
    #[xmlserde(name = b"shadows", ty = "attr")]
    shadows: Option<String>,
    #[xmlserde(name = b"shadowed-by", ty = "attr")]
    shadowed_by: Option<String>,
    #[xmlserde(name = b"throws", ty = "attr")]
    throws: Option<bool>,
    #[xmlserde(name = b"moved-to", ty = "attr")]
    moved_to: Option<String>,
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

    #[xmlserde(name = b"parameters", ty = "child", default = "Parameters::default")]
    parameters: Parameters,
}

impl FunctionMacro {
    pub fn parameters(&self) -> &Parameters {
        &self.parameters
    }
}

impl_info!(FunctionMacro);
impl_attributable!(FunctionMacro);
impl_documentable!(FunctionMacro);
impl_callable!(FunctionMacro);
