use xmlserde_derives::XmlDeserialize;

use crate::{
    attribute::Attribute,
    documentation::{DocDeprecated, DocStability, DocVersion, Documentation, SourcePosition},
    field::Field,
    function::{Function, FunctionInline},
    method::{Method, MethodInline},
    prelude::*,
    union::Union,
    version::Version,
    Callback, Stability,
};

#[derive(Clone, Debug, PartialEq, Eq, XmlDeserialize)]
// FIXME: The `Type` / `AnyType` fields are quite huge and some boxing would
// probably be useful here but `xmlserde` does not seem to support that.
#[allow(clippy::large_enum_variant)]
pub enum RecordField {
    #[xmlserde(name = b"field")]
    Field(Field),
    #[xmlserde(name = b"union")]
    Union(Union),
    #[xmlserde(name = b"record")]
    Record(Record),
    #[xmlserde(name = b"callback")]
    Callback(Callback),
}

#[derive(Clone, Debug, PartialEq, Eq, XmlDeserialize)]
#[xmlserde(root = b"record")]
#[xmlserde(deny_unknown_fields)]
pub struct Record {
    #[xmlserde(name = b"name", ty = "attr")]
    name: Option<String>,
    #[xmlserde(name = b"c:type", ty = "attr")]
    c_type: Option<String>,
    /// Deprecated and replaced by `opaque` & `pointer`
    #[xmlserde(name = b"disguised", ty = "attr")]
    disguised: Option<bool>,
    #[xmlserde(name = b"pointer", ty = "attr")]
    pointer: Option<bool>,
    #[xmlserde(name = b"opaque", ty = "attr")]
    opaque: Option<bool>,
    #[xmlserde(name = b"foreign", ty = "attr")]
    foreign: Option<bool>,
    #[xmlserde(name = b"glib:is-gtype-struct-for", ty = "attr")]
    g_is_gtype_struct_for: Option<String>,
    #[xmlserde(name = b"glib:type-name", ty = "attr")]
    g_type_name: Option<String>,
    #[xmlserde(name = b"glib:get-type", ty = "attr")]
    g_get_type: Option<String>,
    #[xmlserde(name = b"c:symbol-prefix", ty = "attr")]
    symbol_prefix: Option<String>,
    #[xmlserde(name = b"copy-function", ty = "attr")]
    copy_function: Option<String>,
    #[xmlserde(name = b"free-function", ty = "attr")]
    free_function: Option<String>,

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

    #[xmlserde(name = b"constructor", ty = "child")]
    constructors: Vec<Function>,
    #[xmlserde(name = b"function", ty = "child")]
    functions: Vec<Function>,
    #[xmlserde(name = b"function-inline", ty = "child")]
    inline_functions: Vec<FunctionInline>,

    #[xmlserde(name = b"method", ty = "child")]
    methods: Vec<Method>,
    #[xmlserde(name = b"inline-methods", ty = "child")]
    inline_methods: Vec<MethodInline>,

    #[xmlserde(ty = "untag")]
    fields: Vec<RecordField>,
}

impl Record {
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn c_type(&self) -> Option<&str> {
        self.c_type.as_deref()
    }

    pub fn is_disguised(&self) -> bool {
        self.disguised.unwrap_or(false)
    }

    pub fn is_opaque(&self) -> bool {
        self.opaque.unwrap_or(false)
    }

    pub fn is_pointer(&self) -> bool {
        self.pointer.unwrap_or(false)
    }

    pub fn is_foreign(&self) -> bool {
        self.foreign.unwrap_or(false)
    }

    pub fn g_is_gtype_struct_for(&self) -> Option<&str> {
        self.g_is_gtype_struct_for.as_deref()
    }

    pub fn g_type_name(&self) -> Option<&str> {
        self.g_type_name.as_deref()
    }

    pub fn g_get_type(&self) -> Option<&str> {
        self.g_get_type.as_deref()
    }

    pub fn symbol_prefix(&self) -> Option<&str> {
        self.symbol_prefix.as_deref()
    }

    pub fn copy_function(&self) -> Option<&str> {
        self.copy_function.as_deref()
    }

    pub fn free_function(&self) -> Option<&str> {
        self.free_function.as_deref()
    }

    pub fn fields(&self) -> &[RecordField] {
        &self.fields
    }

    pub fn methods(&self) -> &[Method] {
        &self.methods
    }

    pub fn inlined_methods(&self) -> &[MethodInline] {
        &self.inline_methods
    }

    pub fn functions(&self) -> &[Function] {
        &self.functions
    }

    pub fn inlined_functions(&self) -> &[FunctionInline] {
        &self.inline_functions
    }

    pub fn constructors(&self) -> &[Function] {
        &self.constructors
    }
}

impl_info!(Record);
impl_attributable!(Record);
impl_documentable!(Record);
