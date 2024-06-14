use xmlserde_derives::XmlDeserialize;

#[derive(Clone, Debug, XmlDeserialize)]
#[xmlserde(root = b"source-position")]
#[xmlserde(deny_unknown_fields)]
pub struct SourcePosition {
    #[xmlserde(name = b"filename", ty = "attr")]
    filename: String,
    #[xmlserde(name = b"line", ty = "attr")]
    line: String,
    #[xmlserde(name = b"column", ty = "attr")]
    column: Option<String>,
}

impl SourcePosition {
    pub fn filename(&self) -> &str {
        &self.filename
    }

    pub fn line(&self) -> &str {
        &self.line
    }

    pub fn column(&self) -> Option<&str> {
        self.column.as_deref()
    }
}

#[derive(Clone, Debug, XmlDeserialize)]
#[xmlserde(root = b"doc-deprecated")]
#[xmlserde(deny_unknown_fields)]
pub struct DocDeprecated {
    #[xmlserde(name = b"xml:space", ty = "attr")]
    space: Option<String>,
    #[xmlserde(name = b"xml:whitespace", ty = "attr")]
    whitespace: Option<String>,
    #[xmlserde(ty = "text")]
    text: String,
}

impl DocDeprecated {
    pub fn space(&self) -> Option<&str> {
        self.space.as_deref()
    }

    pub fn whitespace(&self) -> Option<&str> {
        self.whitespace.as_deref()
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}

#[derive(Clone, Debug, XmlDeserialize)]
#[xmlserde(root = b"doc-stability")]
#[xmlserde(deny_unknown_fields)]
pub struct DocStability {
    #[xmlserde(name = b"xml:space", ty = "attr")]
    space: Option<String>,
    #[xmlserde(name = b"xml:whitespace", ty = "attr")]
    whitespace: Option<String>,
    #[xmlserde(ty = "text")]
    text: String,
}

impl DocStability {
    pub fn space(&self) -> Option<&str> {
        self.space.as_deref()
    }

    pub fn whitespace(&self) -> Option<&str> {
        self.whitespace.as_deref()
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}

#[derive(Clone, Debug, XmlDeserialize)]
#[xmlserde(root = b"doc-version")]
#[xmlserde(deny_unknown_fields)]
pub struct DocVersion {
    #[xmlserde(name = b"xml:space", ty = "attr")]
    space: Option<String>,
    #[xmlserde(name = b"xml:whitespace", ty = "attr")]
    whitespace: Option<String>,
    #[xmlserde(ty = "text")]
    text: String,
}

impl DocVersion {
    pub fn space(&self) -> Option<&str> {
        self.space.as_deref()
    }

    pub fn whitespace(&self) -> Option<&str> {
        self.whitespace.as_deref()
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}

#[derive(Clone, Debug, XmlDeserialize)]
#[xmlserde(root = b"doc")]
#[xmlserde(deny_unknown_fields)]
pub struct Documentation {
    #[xmlserde(name = b"xml:space", ty = "attr")]
    space: Option<String>,
    #[xmlserde(name = b"xml:whitespace", ty = "attr")]
    whitespace: Option<String>,
    #[xmlserde(name = b"filename", ty = "attr")]
    filename: Option<String>,
    #[xmlserde(name = b"line", ty = "attr")]
    line: Option<String>,
    #[xmlserde(name = b"column", ty = "attr")]
    column: Option<String>,
    #[xmlserde(ty = "text")]
    text: String,
}

impl Documentation {
    pub fn space(&self) -> Option<&str> {
        self.space.as_deref()
    }

    pub fn whitespace(&self) -> Option<&str> {
        self.whitespace.as_deref()
    }

    pub fn filename(&self) -> Option<&str> {
        self.filename.as_deref()
    }

    pub fn line(&self) -> Option<&str> {
        self.line.as_deref()
    }

    pub fn column(&self) -> Option<&str> {
        self.column.as_deref()
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}
