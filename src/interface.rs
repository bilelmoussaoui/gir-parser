use xmlserde_derives::XmlDeserialize;

use crate::{
    attribute::Attribute,
    callback::Callback,
    class::Implements,
    constant::Constant,
    documentation::{DocDeprecated, DocStability, DocVersion, Documentation, SourcePosition},
    field::Field,
    function::{Function, FunctionInline},
    method::{Method, MethodInline},
    prelude::*,
    property::Property,
    signal::Signal,
    version::Version,
    virtual_method::VirtualMethod,
    Stability,
};

#[derive(Debug, XmlDeserialize)]
pub struct Prerequisite {
    #[xmlserde(name = b"name", ty = "attr")]
    name: String,
}

impl Prerequisite {
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Debug, XmlDeserialize)]
pub struct Interface {
    #[xmlserde(name = b"name", ty = "attr")]
    name: String,
    #[xmlserde(name = b"c:symbol-prefix", ty = "attr")]
    symbol_prefix: Option<String>,
    #[xmlserde(name = b"c:type", ty = "attr")]
    c_type: Option<String>,
    #[xmlserde(name = b"glib:type-name", ty = "attr")]
    g_type_name: String,
    #[xmlserde(name = b"glib:get-type", ty = "attr")]
    g_get_type: String,
    #[xmlserde(name = b"glib:type-struct", ty = "attr")]
    g_type_struct: Option<String>,
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

    #[xmlserde(name = b"prerequisite", ty = "child")]
    prerequisites: Vec<Prerequisite>,

    #[xmlserde(name = b"implements", ty = "child")]
    implements: Vec<Implements>,

    #[xmlserde(name = b"function", ty = "child")]
    functions: Vec<Function>,

    #[xmlserde(name = b"function-inline", ty = "child")]
    inline_functions: Vec<FunctionInline>,

    #[xmlserde(name = b"constructor", ty = "child")]
    constructors: Vec<Function>,

    #[xmlserde(name = b"method", ty = "child")]
    methods: Vec<Method>,

    #[xmlserde(name = b"inline-methods", ty = "child")]
    inline_methods: Vec<MethodInline>,

    #[xmlserde(name = b"virtual-method", ty = "child")]
    virtual_methods: Vec<VirtualMethod>,

    #[xmlserde(name = b"field", ty = "child")]
    fields: Vec<Field>,

    #[xmlserde(name = b"property", ty = "child")]
    properties: Vec<Property>,

    #[xmlserde(name = b"signal", ty = "child")]
    signals: Vec<Signal>,

    #[xmlserde(name = b"callback", ty = "child")]
    callbacks: Vec<Callback>,

    #[xmlserde(name = b"constant", ty = "child")]
    constants: Vec<Constant>,
}

impl Interface {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn symbol_prefix(&self) -> Option<&str> {
        self.symbol_prefix.as_deref()
    }

    pub fn c_type(&self) -> Option<&str> {
        self.c_type.as_deref()
    }

    pub fn g_type_name(&self) -> &str {
        &self.g_type_name
    }

    pub fn g_get_type(&self) -> &str {
        &self.g_get_type
    }

    pub fn g_type_struct(&self) -> Option<&str> {
        self.g_type_struct.as_deref()
    }

    pub fn prerequisites(&self) -> &[Prerequisite] {
        &self.prerequisites
    }

    pub fn implements(&self) -> &[Implements] {
        &self.implements
    }

    pub fn constructors(&self) -> &[Function] {
        &self.constructors
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

    pub fn virtual_methods(&self) -> &[VirtualMethod] {
        &self.virtual_methods
    }

    pub fn fields(&self) -> &[Field] {
        &self.fields
    }

    pub fn properties(&self) -> &[Property] {
        &self.properties
    }

    pub fn signals(&self) -> &[Signal] {
        &self.signals
    }

    pub fn constants(&self) -> &[Constant] {
        &self.constants
    }

    pub fn callbacks(&self) -> &[Callback] {
        &self.callbacks
    }
}

impl_documentable!(Interface);
impl_attributable!(Interface);
impl_info!(Interface);
