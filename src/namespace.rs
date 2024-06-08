use xmlserde_derives::XmlDeserialize;

use crate::{
    alias::Alias,
    attribute::Attribute,
    bitfield::BitField,
    boxed::Boxed,
    callback::Callback,
    class::Class,
    constant::Constant,
    enums::Enumeration,
    function::{Function, FunctionInline},
    function_macro::FunctionMacro,
    interface::Interface,
    prelude::*,
    record::Record,
    union::Union,
    version::Version,
};

#[derive(Clone, Debug, XmlDeserialize)]
#[cfg_attr(test, derive(Default))]
#[xmlserde(root = b"namespace")]
pub struct Namespace {
    #[xmlserde(name = b"name", ty = "attr")]
    name: String,
    #[xmlserde(name = b"version", ty = "attr")]
    version: Version,
    #[xmlserde(name = b"c:identifier-prefixes", ty = "attr")]
    c_identifier_prefixes: Option<String>,
    // Deprecated, backwards compatibility only
    #[xmlserde(name = b"c:prefix", ty = "attr")]
    c_prefix: Option<String>,
    #[xmlserde(name = b"c:symbol-prefixes", ty = "attr")]
    c_symbol_prefixes: Option<String>,
    #[xmlserde(name = b"shared-library", ty = "attr")]
    shared_library: Option<String>,

    #[xmlserde(name = b"alias", ty = "child")]
    aliases: Vec<Alias>,
    #[xmlserde(name = b"class", ty = "child")]
    classes: Vec<Class>,
    #[xmlserde(name = b"interface", ty = "child")]
    interfaces: Vec<Interface>,
    #[xmlserde(name = b"record", ty = "child")]
    records: Vec<Record>,
    #[xmlserde(name = b"enumeration", ty = "child")]
    enums: Vec<Enumeration>,
    #[xmlserde(name = b"function", ty = "child")]
    functions_global: Vec<Function>,
    #[xmlserde(name = b"function-inline", ty = "child")]
    inline_functions: Vec<FunctionInline>,
    #[xmlserde(name = b"function-macro", ty = "child")]
    functions_macro: Vec<FunctionMacro>,
    #[xmlserde(name = b"union", ty = "child")]
    unions: Vec<Union>,
    #[xmlserde(name = b"bitfield", ty = "child")]
    flags: Vec<BitField>,
    #[xmlserde(name = b"callback", ty = "child")]
    callbacks: Vec<Callback>,
    #[xmlserde(name = b"constant", ty = "child")]
    constants: Vec<Constant>,
    // Attributes: 0 or more
    #[xmlserde(name = b"attribute", ty = "child")]
    attributes: Vec<Attribute>,
    #[xmlserde(name = b"glib:boxed", ty = "child")]
    boxed: Vec<Boxed>,
}

impl Namespace {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn version(&self) -> &Version {
        &self.version
    }

    pub fn c_identifier_prefixes(&self) -> &str {
        self.c_identifier_prefixes
            .as_ref()
            .or(self.c_prefix.as_ref())
            .unwrap()
    }

    pub fn c_symbol_prefixes(&self) -> Option<&str> {
        self.c_symbol_prefixes.as_deref()
    }

    pub fn shared_library(&self) -> Option<&str> {
        self.shared_library.as_deref()
    }

    pub fn aliases(&self) -> &[Alias] {
        &self.aliases
    }

    pub fn constants(&self) -> &[Constant] {
        &self.constants
    }

    pub fn functions(&self) -> &[Function] {
        &self.functions_global
    }

    pub fn inlined_functions(&self) -> &[FunctionInline] {
        &self.inline_functions
    }

    pub fn macros(&self) -> &[FunctionMacro] {
        &self.functions_macro
    }

    pub fn enums(&self) -> &[Enumeration] {
        &self.enums
    }

    pub fn flags(&self) -> &[BitField] {
        &self.flags
    }

    pub fn unions(&self) -> &[Union] {
        &self.unions
    }

    pub fn boxed(&self) -> &[Boxed] {
        &self.boxed
    }

    pub fn records(&self) -> &[Record] {
        &self.records
    }

    pub fn classes(&self) -> &[Class] {
        &self.classes
    }

    pub fn callbacks(&self) -> &[Callback] {
        &self.callbacks
    }

    pub fn interfaces(&self) -> &[Interface] {
        &self.interfaces
    }

    /// Copied from the old gir
    pub fn link_name(&self) -> Option<&str> {
        let mut s = self.shared_library.as_deref()?;

        if s.starts_with("lib") {
            s = &s[3..];
        }

        if let Some(offset) = s.rfind(".so") {
            s = &s[..offset];
        } else if let Some(offset) = s.rfind(".dll") {
            s = &s[..offset];
            if let Some(offset) = s.rfind('-') {
                s = &s[..offset];
            }
        }

        Some(s)
    }
}

impl_attributable!(Namespace);

#[cfg(test)]
mod tests {
    #[test]
    fn shared_library_to_link_name() {
        let mut namespace = super::Namespace::default();
        let tests = [
            ("libgtk-4-1.dll", "gtk-4"),
            ("libatk-1.0.so.0", "atk-1.0"),
            ("libgdk_pixbuf-2.0.so.0", "gdk_pixbuf-2.0"),
        ];
        for (shared_lib, expected_result) in tests {
            namespace.shared_library = Some(shared_lib.to_owned());
            assert_eq!(namespace.link_name(), Some(expected_result));
        }
    }
}
