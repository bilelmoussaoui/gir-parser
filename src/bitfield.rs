use xmlserde_derives::XmlDeserialize;

use crate::{
    attribute::Attribute,
    documentation::{DocDeprecated, DocStability, DocVersion, Documentation, SourcePosition},
    function::{Function, FunctionInline},
    member::Member,
    prelude::*,
    version::Version,
    Stability,
};

#[derive(Clone, Debug, PartialEq, Eq, XmlDeserialize)]
#[xmlserde(root = b"bitfield")]
#[xmlserde(deny_unknown_fields)]
pub struct BitField {
    #[xmlserde(name = b"name", ty = "attr")]
    name: String,
    #[xmlserde(name = b"c:type", ty = "attr")]
    c_type: String,
    #[xmlserde(name = b"glib:get-type", ty = "attr")]
    g_get_type: Option<String>,
    #[xmlserde(name = b"glib:type-name", ty = "attr")]
    g_type_name: Option<String>,
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
    // Members
    #[xmlserde(name = b"member", ty = "child")]
    members: Vec<Member>,
    // Functions
    #[xmlserde(name = b"function", ty = "child")]
    functions: Vec<Function>,
    #[xmlserde(name = b"function-inline", ty = "child")]
    inline_functions: Vec<FunctionInline>,
}

impl BitField {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn c_type(&self) -> &str {
        &self.c_type
    }

    pub fn g_get_type(&self) -> Option<&str> {
        self.g_get_type.as_deref()
    }

    pub fn g_type_name(&self) -> Option<&str> {
        self.g_type_name.as_deref()
    }

    pub fn members(&self) -> &[Member] {
        &self.members
    }

    pub fn functions(&self) -> &[Function] {
        &self.functions
    }

    pub fn inlined_functions(&self) -> &[FunctionInline] {
        &self.inline_functions
    }
}

impl_info!(BitField);
impl_attributable!(BitField);
impl_documentable!(BitField);

impl IntoIterator for BitField {
    type Item = Member;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.members.into_iter()
    }
}

impl<'a> IntoIterator for &'a BitField {
    type Item = &'a Member;
    type IntoIter = std::slice::Iter<'a, Member>;

    fn into_iter(self) -> Self::IntoIter {
        self.members.iter()
    }
}

impl AsRef<[Member]> for BitField {
    fn as_ref(&self) -> &[Member] {
        &self.members
    }
}

impl std::ops::Deref for BitField {
    type Target = [Member];

    fn deref(&self) -> &Self::Target {
        &self.members
    }
}

impl std::ops::Index<usize> for BitField {
    type Output = Member;

    fn index(&self, index: usize) -> &Self::Output {
        &self.members[index]
    }
}
