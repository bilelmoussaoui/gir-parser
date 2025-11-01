use xmlserde_derives::XmlDeserialize;

use crate::{
    attribute::Attribute,
    documentation::{DocDeprecated, DocStability, DocVersion, Documentation, SourcePosition},
    prelude::*,
    r#type::AnyType,
    FunctionScope, TransferOwnership,
};

#[derive(Clone, Debug, XmlDeserialize)]
#[xmlserde(root = b"return-value")]
#[xmlserde(deny_unknown_fields)]
pub struct ReturnValue {
    #[xmlserde(name = b"introspectable", ty = "attr")]
    introspectable: Option<bool>,
    #[xmlserde(name = b"nullable", ty = "attr")]
    nullable: Option<bool>,
    #[xmlserde(name = b"closure", ty = "attr")]
    closure: Option<usize>,
    #[xmlserde(name = b"scope", ty = "attr")]
    scope: Option<FunctionScope>,
    #[xmlserde(name = b"destroy", ty = "attr")]
    destroy: Option<usize>,
    #[xmlserde(name = b"skip", ty = "attr")]
    skip: Option<bool>,
    #[xmlserde(name = b"allow-none", ty = "attr")]
    allow_none: Option<bool>,

    #[xmlserde(name = b"transfer-ownership", ty = "attr")]
    transfer: Option<TransferOwnership>,

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
    type_: AnyType,
}

impl ReturnValue {
    pub fn is_introspectable(&self) -> bool {
        self.introspectable.unwrap_or(true)
    }

    pub fn is_nullable(&self) -> Option<bool> {
        self.nullable
    }

    pub fn closure(&self) -> Option<usize> {
        self.closure
    }

    pub fn scope(&self) -> Option<FunctionScope> {
        self.scope
    }

    pub fn destroy(&self) -> Option<usize> {
        self.destroy
    }

    pub fn is_skip(&self) -> Option<bool> {
        self.skip
    }

    pub fn is_allow_none(&self) -> Option<bool> {
        self.allow_none
    }

    pub fn transfer_ownership(&self) -> Option<TransferOwnership> {
        self.transfer
    }

    pub fn ty(&self) -> &AnyType {
        &self.type_
    }
}

impl_documentable!(ReturnValue);
impl_attributable!(ReturnValue);
