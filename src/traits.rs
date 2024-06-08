use crate::{
    attribute::Attribute,
    documentation::{DocDeprecated, DocStability, DocVersion, Documentation, SourcePosition},
    version::Version,
    Stability,
};

pub trait Documentable {
    fn doc(&self) -> Option<&Documentation>;
    fn doc_deprecated(&self) -> Option<&DocDeprecated>;
    fn doc_stability(&self) -> Option<&DocStability>;
    fn doc_version(&self) -> Option<&DocVersion>;
    fn source_position(&self) -> Option<&SourcePosition>;
}

pub trait Attributable {
    fn attributes(&self) -> &[Attribute];

    fn gtk_property_get(&self) -> Option<&str> {
        self.attributes()
            .iter()
            .find(|a| a.name() == "org.gtk.Property.get")
            .map(|a| a.value())
    }

    fn gtk_method_get_property(&self) -> Option<&str> {
        self.attributes()
            .iter()
            .find(|a| a.name() == "org.gtk.Method.get_property")
            .map(|a| a.value())
    }

    fn gtk_property_set(&self) -> Option<&str> {
        self.attributes()
            .iter()
            .find(|a| a.name() == "org.gtk.Property.set")
            .map(|a| a.value())
    }

    fn gtk_method_set_property(&self) -> Option<&str> {
        self.attributes()
            .iter()
            .find(|a| a.name() == "org.gtk.Method.set_property")
            .map(|a| a.value())
    }
}

pub trait Info: Documentable + Attributable {
    fn is_introspectable(&self) -> bool;
    fn is_deprecated(&self) -> bool;
    fn version(&self) -> Option<&Version>;
    fn deprecated_version(&self) -> Option<&Version>;
    fn stability(&self) -> Option<Stability>;
}

pub trait Callable: Info {
    fn name(&self) -> &str;
    fn c_identifier(&self) -> Option<&str>;
    fn shadows(&self) -> Option<&str>;
    fn shadowed_by(&self) -> Option<&str>;
    fn throws(&self) -> bool;
    fn moved_to(&self) -> Option<&str>;
    fn async_func(&self) -> Option<&str>;
    fn finish_func(&self) -> Option<&str>;
    fn sync_func(&self) -> Option<&str>;
}

macro_rules! impl_documentable {
    ($rust_type:ident) => {
        impl Documentable for $rust_type {
            fn doc(&self) -> Option<&Documentation> {
                self.doc.as_ref()
            }
            fn doc_deprecated(&self) -> Option<&DocDeprecated> {
                self.doc_deprecated.as_ref()
            }
            fn doc_stability(&self) -> Option<&DocStability> {
                self.doc_stability.as_ref()
            }
            fn doc_version(&self) -> Option<&DocVersion> {
                self.doc_version.as_ref()
            }
            fn source_position(&self) -> Option<&SourcePosition> {
                self.source_position.as_ref()
            }
        }
    };
}

macro_rules! impl_attributable {
    ($rust_type:ident) => {
        impl Attributable for $rust_type {
            fn attributes(&self) -> &[Attribute] {
                &self.attributes
            }
        }
    };
}

macro_rules! impl_info {
    ($rust_type:ident) => {
        impl Info for $rust_type {
            fn is_introspectable(&self) -> bool {
                self.introspectable.unwrap_or(true)
            }

            fn is_deprecated(&self) -> bool {
                self.deprecated.unwrap_or(false)
            }

            fn version(&self) -> Option<&Version> {
                self.version.as_ref()
            }

            fn deprecated_version(&self) -> Option<&Version> {
                self.deprecated_version.as_ref()
            }

            fn stability(&self) -> Option<Stability> {
                self.stability
            }
        }
    };
}

macro_rules! impl_callable {
    ($rust_type:ident) => {
        impl Callable for $rust_type {
            fn name(&self) -> &str {
                &self.name
            }

            fn c_identifier(&self) -> Option<&str> {
                self.c_identifier.as_deref()
            }

            fn shadows(&self) -> Option<&str> {
                self.shadows.as_deref()
            }

            fn shadowed_by(&self) -> Option<&str> {
                self.shadowed_by.as_deref()
            }

            fn throws(&self) -> bool {
                self.throws.unwrap_or(false)
            }

            fn moved_to(&self) -> Option<&str> {
                self.moved_to.as_deref()
            }

            fn async_func(&self) -> Option<&str> {
                self.async_func.as_deref()
            }

            fn finish_func(&self) -> Option<&str> {
                self.finish_func.as_deref()
            }

            fn sync_func(&self) -> Option<&str> {
                self.sync_func.as_deref()
            }
        }
    };
}
