use xmlserde_derives::XmlDeserialize;

use crate::{
    attribute::Attribute,
    documentation::{DocDeprecated, DocStability, DocVersion, Documentation, SourcePosition},
    parameter::Parameters,
    prelude::*,
    return_value::ReturnValue,
    version::Version,
    SignalEmission, Stability,
};

#[derive(Clone, Debug, PartialEq, Eq, XmlDeserialize)]
#[xmlserde(root = b"glib:signal")]
#[xmlserde(deny_unknown_fields)]
pub struct Signal {
    #[xmlserde(name = b"name", ty = "attr")]
    name: String,
    #[xmlserde(name = b"detailed", ty = "attr")]
    detailed: Option<bool>,
    #[xmlserde(name = b"when", ty = "attr")]
    when: Option<SignalEmission>,
    #[xmlserde(name = b"action", ty = "attr")]
    action: Option<bool>,
    #[xmlserde(name = b"no-hooks", ty = "attr")]
    no_hooks: Option<bool>,
    #[xmlserde(name = b"no-recurse", ty = "attr")]
    no_recurse: Option<bool>,
    #[xmlserde(name = b"emitter", ty = "attr")]
    emitter: Option<String>,
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

impl Signal {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_detailed(&self) -> bool {
        self.detailed.unwrap_or(false)
    }

    pub fn when(&self) -> Option<SignalEmission> {
        self.when
    }

    pub fn is_action(&self) -> bool {
        self.action.unwrap_or(false)
    }

    pub fn is_no_hooks(&self) -> bool {
        self.no_hooks.unwrap_or(false)
    }

    pub fn is_no_recurse(&self) -> bool {
        self.no_recurse.unwrap_or(false)
    }

    pub fn emitter(&self) -> Option<&str> {
        self.emitter.as_deref()
    }

    pub fn return_value(&self) -> &ReturnValue {
        &self.return_value
    }

    pub fn parameters(&self) -> &Parameters {
        &self.parameters
    }
}

impl_info!(Signal);
impl_attributable!(Signal);
impl_documentable!(Signal);
