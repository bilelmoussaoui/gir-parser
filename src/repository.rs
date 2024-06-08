use std::path::Path;

use xmlserde_derives::XmlDeserialize;

use crate::{namespace::Namespace, version::Version, ParserError};

#[derive(Clone, Debug, XmlDeserialize)]
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
pub struct Package {
    #[xmlserde(name = b"name", ty = "attr")]
    name: String,
}

impl Package {
    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Clone, Debug, XmlDeserialize)]
#[xmlserde(root = b"repository")]
pub struct Repository {
    #[xmlserde(name = b"version", ty = "attr")]
    version: Option<Version>,
    #[xmlserde(name = b"c:identifier-prefixes", ty = "attr")]
    c_identifier_prefixes: Option<String>,
    #[xmlserde(name = b"c:symbol-prefixes", ty = "attr")]
    c_symbol_prefixes: Option<String>,
    #[xmlserde(name = b"include", ty = "child")]
    includes: Vec<NamespaceInclude>,
    #[xmlserde(name = b"c:include", ty = "child")]
    c_includes: Vec<HeaderInclude>,
    #[xmlserde(name = b"package", ty = "child")]
    packages: Vec<Package>,
    #[xmlserde(name = b"namespace", ty = "child")]
    namespace: Namespace,
}

impl Repository {
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
}
