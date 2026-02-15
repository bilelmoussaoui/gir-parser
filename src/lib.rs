#![doc = include_str!("../README.md")]
use xmlserde::xml_serde_enum;

#[derive(Debug)]
pub enum ParserError {
    IO(std::io::Error),
    Xml(String),
}

impl From<std::io::Error> for ParserError {
    fn from(value: std::io::Error) -> Self {
        Self::IO(value)
    }
}

impl std::error::Error for ParserError {}
impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IO(e) => f.write_fmt(format_args!("I/O operation failed {e}")),
            Self::Xml(e) => f.write_fmt(format_args!("Failed to parse xml file: {e}")),
        }
    }
}

xml_serde_enum! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    Stability {
        Stable => "Stable",
        Unstable => "Unstable",
        Private => "Private",
    }
}

xml_serde_enum! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    TransferOwnership {
        None => "none",
        Container => "container",
        Full => "full",
    }
}

impl TransferOwnership {
    pub fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }

    pub fn is_full(&self) -> bool {
        matches!(self, Self::Full)
    }

    pub fn is_container(&self) -> bool {
        matches!(self, Self::Container)
    }
}

xml_serde_enum! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    FunctionScope {
        Call => "call",
        Notified => "notified",
        Async => "async",
        Forever => "forever",
    }
}

impl FunctionScope {
    pub fn is_call(&self) -> bool {
        matches!(self, Self::Call)
    }

    pub fn is_notified(&self) -> bool {
        matches!(self, Self::Notified)
    }

    pub fn is_async(&self) -> bool {
        matches!(self, Self::Async)
    }

    pub fn is_forever(&self) -> bool {
        matches!(self, Self::Forever)
    }
}

xml_serde_enum! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
    SignalEmission {
        First => "first",
        Last => "last",
        Cleanup => "cleanup",
    }
}

#[macro_use]
mod traits;
pub mod prelude {
    pub use xmlserde::XmlValue;

    pub use super::traits::*;
}

mod alias;
pub use alias::Alias;
mod array;
pub use array::Array;
mod attribute;
pub use attribute::Attribute;
mod bitfield;
pub use bitfield::BitField;
mod boxed;
pub use boxed::Boxed;
mod callback;
pub use callback::Callback;
mod class;
pub use class::{Class, ClassField, Implements};
mod constant;
pub use constant::Constant;
mod documentation;
pub use documentation::{DocDeprecated, DocStability, DocVersion, Documentation, SourcePosition};
mod enums;
pub use enums::Enumeration;
mod field;
pub use field::{Field, FieldType};
mod function;
pub use function::{Function, FunctionInline};
mod function_macro;
pub use function_macro::FunctionMacro;
mod interface;
pub use interface::{Interface, InterfaceField, Prerequisite};
mod member;
pub use member::Member;
mod method;
pub use method::{Method, MethodInline};
mod namespace;
pub use namespace::Namespace;
mod parameter;
pub use parameter::{
    AnyParameter, Direction, InstanceParameter, Parameter, ParameterType, Parameters,
};
mod property;
pub use property::Property;
mod record;
pub use record::{Record, RecordField};
mod repository;
pub use repository::{DocFormat, HeaderInclude, NamespaceInclude, Package, Repository};
mod return_value;
pub use return_value::ReturnValue;
mod signal;
pub use signal::Signal;
mod r#type;
pub use r#type::{AnyType, Type};
mod union;
pub use union::{Union, UnionField};
mod version;
pub use version::Version;
mod virtual_method;
pub use virtual_method::VirtualMethod;

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, str::FromStr};

    use super::prelude::*;

    const GIR_FILES: [&str; 35] = [
        "Atk-1.0",
        "cairo-1.0",
        "fontconfig-2.0",
        "freetype2-2.0",
        "Gdk-3.0",
        "Gdk-4.0",
        "GdkPixbuf-2.0",
        "GdkPixdata-2.0",
        "GdkWayland-4.0",
        "GdkWin32-4.0",
        "GdkX11-3.0",
        "GdkX11-4.0",
        "Gio-2.0",
        "GL-1.0",
        "GLib-2.0",
        "GModule-2.0",
        "GObject-2.0",
        "Graphene-1.0",
        "Gsk-4.0",
        "Gtk-3.0",
        "Gtk-4.0",
        "HarfBuzz-0.0",
        "libxml2-2.0",
        "Pango-1.0",
        "PangoCairo-1.0",
        "PangoFc-1.0",
        "PangoFT2-1.0",
        "PangoOT-1.0",
        "PangoXft-1.0",
        "Vulkan-1.0",
        "win32-1.0",
        "xfixes-4.0",
        "xft-2.0",
        "xlib-2.0",
        "xrandr-1.3",
    ];

    use super::{repository::Repository, version::Version};

    fn parse_gir(gir_file: &str) -> Repository {
        let path = PathBuf::from("./gir-files").join(format!("{}.gir", gir_file));
        Repository::from_path(path).unwrap()
    }
    #[test]
    fn xft_gir() {
        let repo = parse_gir(GIR_FILES[32]);
        assert_eq!(repo.version(), Version::from_str("1.2").ok().as_ref());
        assert_eq!(repo.namespace_includes()[0].as_package(), "xlib-2.0");

        let namespace = repo.namespace();
        assert_eq!(namespace.version(), "2.0");
        assert_eq!(namespace.name(), "xft");
        assert_eq!(
            namespace.c_identifier_prefixes().collect::<Vec<_>>(),
            vec!["Xft"]
        );
        assert_eq!(
            namespace.c_symbol_prefixes().collect::<Vec<_>>(),
            vec!["Xft"]
        );
    }

    #[test]
    fn xlib_gir() {
        let repo = parse_gir(GIR_FILES[33]);
        assert_eq!(repo.version(), Version::from_str("1.2").ok().as_ref());
        let namespace = repo.namespace();
        assert_eq!(namespace.version(), "2.0");
        assert_eq!(namespace.name(), "xlib");
        assert!(namespace
            .c_identifier_prefixes()
            .collect::<Vec<_>>()
            .is_empty());
        assert_eq!(namespace.c_symbol_prefixes().collect::<Vec<_>>(), vec!["X"]);
        let aliases = namespace.aliases();
        assert_eq!(aliases[0].name(), "Atom");
        assert_eq!(aliases[0].c_type(), "Atom");
        assert_eq!(aliases[0].ty().as_type().name(), Some("gulong"));
        assert_eq!(aliases[0].ty().as_type().c_type(), Some("gulong"));

        let records = namespace.records();
        assert_eq!(records[0].name(), Some("Display"));
        assert_eq!(records[0].c_type(), Some("Display"));

        let unions = namespace.unions();
        assert_eq!(unions[0].name(), Some("XEvent"));
        assert_eq!(unions[0].c_type(), Some("XEvent"));

        let functions = namespace.functions();
        assert_eq!(functions[0].name(), "open_display");
        assert_eq!(functions[0].c_identifier(), Some("XOpenDisplay"));
        assert!(functions[0].parameters().is_empty());

        let return_value = functions[0].return_value();
        assert!(return_value.transfer_ownership().unwrap().is_none());
        assert_eq!(return_value.ty().as_type().name(), Some("none"));
        assert_eq!(return_value.ty().as_type().c_type(), Some("void"));
    }

    #[test]
    fn xrandr_gir() {
        let repo = parse_gir(GIR_FILES[34]);
        assert_eq!(repo.version(), Version::from_str("1.2").ok().as_ref());
        let namespace = repo.namespace();
        assert_eq!(namespace.version(), "1.3");
        assert_eq!(namespace.name(), "xrandr");
        assert_eq!(
            namespace.c_identifier_prefixes().collect::<Vec<_>>(),
            vec!["XRR"]
        );
        assert_eq!(
            namespace.c_symbol_prefixes().collect::<Vec<_>>(),
            vec!["XRR"]
        );
        let records = namespace.records();
        assert_eq!(records[0].name(), Some("ScreenSize"));
        assert_eq!(records[0].c_type(), Some("XRRScreenSize"));
    }

    #[test]
    fn parse_all_gir_files() {
        let paths = std::fs::read_dir("./gir-files").unwrap();

        for path in paths {
            let path = path.unwrap().path();
            let gir_file = path
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .trim_end_matches(".gir")
                .to_owned();
            println!("{:#?}", gir_file);
            let repository = parse_gir(&gir_file);
            assert!(gir_file.starts_with(repository.namespace().name()));
        }
    }

    #[test]
    fn parse_doc_format_is_missing() {
        // doc:format is missing
        let content = r#"
<repository xmlns="http://www.gtk.org/introspection/core/1.0" xmlns:c="http://www.gtk.org/introspection/c/1.0" xmlns:doc="http://www.gtk.org/introspection/doc/1.0" xmlns:glib="http://www.gtk.org/introspection/glib/1.0" version="1.2">
  <include name="GObject" version="2.0"/>
  <package name="pango"/>
  <c:include name="pango/pango.h"/>
  <namespace name="Pango" version="1.0" shared-library="libpango-1.0.so.0" c:identifier-prefixes="Pango" c:symbol-prefixes="pango">
  </namespace>
</repository>"#;
        let repo = Repository::from_str(content).unwrap();
        assert_eq!(repo.doc_format(), crate::DocFormat::Unknown);
    }

    #[test]
    fn parse_doc_format_gi_docgen_missing() {
        // doc:format is missing
        let content = r#"
<repository xmlns="http://www.gtk.org/introspection/core/1.0" xmlns:c="http://www.gtk.org/introspection/c/1.0" xmlns:doc="http://www.gtk.org/introspection/doc/1.0" xmlns:glib="http://www.gtk.org/introspection/glib/1.0" version="1.2">
  <include name="GObject" version="2.0"/>
  <package name="pango"/>
  <c:include name="pango/pango.h"/>
  <doc:format name="gi-docgen"/>
  <namespace name="Pango" version="1.0" shared-library="libpango-1.0.so.0" c:identifier-prefixes="Pango" c:symbol-prefixes="pango">
  </namespace>
</repository>"#;
        let repo = Repository::from_str(content).unwrap();
        assert_eq!(repo.doc_format(), crate::DocFormat::GiDocgen);
    }
}
