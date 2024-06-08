use xmlserde_derives::XmlDeserialize;

use crate::{
    attribute::Attribute,
    documentation::{DocDeprecated, DocStability, DocVersion, Documentation, SourcePosition},
    parameter::Parameters,
    prelude::*,
    return_value::ReturnValue,
    version::Version,
    Stability,
};

#[derive(Debug, XmlDeserialize)]
#[xmlserde(root = b"method")]
pub struct Method {
    #[xmlserde(name = b"glib:get-property", ty = "attr")]
    get_property: Option<String>,
    #[xmlserde(name = b"glib:set-property", ty = "attr")]
    set_property: Option<String>,

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
    #[xmlserde(name = b"return-value", ty = "child")]
    return_value: ReturnValue,
    #[xmlserde(name = b"parameters", ty = "child", default = "Parameters::default")]
    parameters: Parameters,
}

impl Method {
    pub fn get_property(&self) -> Option<&str> {
        self.get_property.as_deref()
    }

    pub fn set_property(&self) -> Option<&str> {
        self.set_property.as_deref()
    }

    pub fn return_value(&self) -> &ReturnValue {
        &self.return_value
    }

    pub fn parameters(&self) -> &Parameters {
        &self.parameters
    }
}

impl_info!(Method);
impl_attributable!(Method);
impl_documentable!(Method);
impl_callable!(Method);

#[derive(Debug, XmlDeserialize)]
pub struct MethodInline {
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

    #[xmlserde(name = b"return-value", ty = "child")]
    return_value: ReturnValue,
    #[xmlserde(name = b"parameters", ty = "child", default = "Parameters::default")]
    parameters: Parameters,
}

impl MethodInline {
    pub fn return_value(&self) -> &ReturnValue {
        &self.return_value
    }

    pub fn parameters(&self) -> &Parameters {
        &self.parameters
    }
}

impl_info!(MethodInline);
impl_attributable!(MethodInline);
impl_documentable!(MethodInline);
impl_callable!(MethodInline);
