use std::{collections::HashMap, path::Path};

use xmlserde_derives::XmlDeserialize;

use crate::{namespace::Namespace, version::Version, ParserError};

#[derive(Clone, Debug, XmlDeserialize)]
#[xmlserde(root = b"include")]
#[xmlserde(deny_unknown_fields)]
pub struct NamespaceInclude {
    #[xmlserde(name = b"name", ty = "attr")]
    name: String,
    #[xmlserde(name = b"version", ty = "attr")]
    version: Version,
}

impl NamespaceInclude {
    pub fn as_package(&self) -> String {
        format!("{}-{}", self.name, self.version)
    }

    pub fn as_package_file(&self) -> String {
        format!("{}-{}.gir", self.name, self.version)
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn version(&self) -> &Version {
        &self.version
    }
}

#[derive(Clone, Debug, XmlDeserialize)]
#[xmlserde(root = b"c:include")]
#[xmlserde(deny_unknown_fields)]
pub struct HeaderInclude {
    #[xmlserde(name = b"name", ty = "attr")]
    name: String,
}

impl HeaderInclude {
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Clone, Debug, XmlDeserialize)]
#[xmlserde(root = b"package")]
#[xmlserde(deny_unknown_fields)]
pub struct Package {
    #[xmlserde(name = b"name", ty = "attr")]
    name: String,
}

impl Package {
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Clone, Debug)]
pub enum DocFormat {
    GtkDoc,
    GiDocgen,
    Other(String),
}

impl xmlserde::XmlValue for DocFormat {
    fn serialize(&self) -> String {
        match self {
            Self::GtkDoc => "gtk-doc".to_owned(),
            Self::GiDocgen => "gi-docgen".to_owned(),
            Self::Other(e) => e.to_owned(),
        }
    }

    fn deserialize(s: &str) -> Result<Self, String> {
        match s {
            "gtk-doc" => Ok(Self::GtkDoc),
            "gi-docgen" => Ok(Self::GiDocgen),
            e => Ok(Self::Other(e.to_owned())),
        }
    }
}

#[derive(Clone, Debug, XmlDeserialize)]
#[xmlserde(root = b"repository")]
#[xmlserde(deny_unknown_fields)]
pub struct Repository {
    #[xmlserde(name = b"version", ty = "attr")]
    version: Option<Version>,
    #[xmlserde(name = b"c:identifier-prefixes", ty = "attr")]
    c_identifier_prefixes: Option<String>,
    #[xmlserde(name = b"c:symbol-prefixes", ty = "attr")]
    c_symbol_prefixes: Option<String>,
    #[xmlserde(name = b"xmlns", ty = "attr")]
    _xmlns: Option<String>,
    #[xmlserde(name = b"xmlns:c", ty = "attr")]
    _xmlns_c: Option<String>,
    #[xmlserde(name = b"xmlns:glib", ty = "attr")]
    _xmlns_glib: Option<String>,
    #[xmlserde(name = b"xmlns:doc", ty = "attr")]
    _xmlns_doc: Option<String>,
    #[xmlserde(name = b"include", ty = "child")]
    includes: Vec<NamespaceInclude>,
    #[xmlserde(name = b"c:include", ty = "child")]
    c_includes: Vec<HeaderInclude>,
    #[xmlserde(name = b"package", ty = "child")]
    packages: Vec<Package>,
    #[xmlserde(name = b"namespace", ty = "child")]
    namespace: Namespace,
    #[xmlserde(name = b"doc:format", ty = "attr")]
    doc_format: Option<DocFormat>,
}

impl Repository {
    pub fn from_path_follow_namespaces_and_cache(
        cache: &mut HashMap<String, Self>,
        package_file: &str,
        girs_dirs: impl AsRef<Path>,
    ) -> Result<(), ParserError> {
        let repo = Self::from_path(girs_dirs.as_ref().join(package_file))?;
        if cache.contains_key(package_file) {
            return Ok(());
        }
        for namespace in repo.namespace_includes() {
            if !cache.contains_key(&namespace.as_package_file()) {
                Self::from_path_follow_namespaces_and_cache(
                    cache,
                    &namespace.as_package_file(),
                    girs_dirs.as_ref(),
                )?;
            }
        }
        debug_assert_eq!(
            package_file,
            format!(
                "{}-{}.gir",
                repo.namespace().name(),
                repo.namespace().version()
            )
        );
        cache.insert(package_file.to_owned(), repo);
        Ok(())
    }

    pub fn from_path_follow_namespaces(
        package_file: &str,
        girs_dirs: impl AsRef<Path>,
    ) -> Result<HashMap<String, Self>, ParserError> {
        let mut output = HashMap::new();
        Self::from_path_follow_namespaces_and_cache(&mut output, package_file, girs_dirs)?;
        Ok(output)
    }

    pub fn from_path(path: impl AsRef<Path>) -> Result<Self, ParserError> {
        let content = std::fs::read_to_string(path)?;
        let repository = xmlserde::xml_deserialize_from_str(&content).map_err(ParserError::Xml)?;
        Ok(repository)
    }

    pub fn version(&self) -> Option<&Version> {
        self.version.as_ref()
    }

    // TODO: split this by ","
    pub fn c_identifier_prefixes(&self) -> Option<&str> {
        self.c_identifier_prefixes.as_deref()
    }

    // TODO: split this by ","
    pub fn c_symbol_prefixes(&self) -> Option<&str> {
        self.c_symbol_prefixes.as_deref()
    }

    pub fn namespace_includes(&self) -> &[NamespaceInclude] {
        &self.includes
    }

    pub fn header_includes(&self) -> &[HeaderInclude] {
        &self.c_includes
    }

    pub fn packages(&self) -> &[Package] {
        &self.packages
    }

    pub fn namespace(&self) -> &Namespace {
        &self.namespace
    }

    pub fn doc_format(&self) -> Option<&DocFormat> {
        self.doc_format.as_ref()
    }
}
