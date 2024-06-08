use xmlserde_derives::XmlDeserialize;

#[derive(Debug, XmlDeserialize)]
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

#[derive(Debug, XmlDeserialize)]
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

#[derive(Debug, XmlDeserialize)]
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

#[derive(Debug, XmlDeserialize)]
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

#[derive(Debug, XmlDeserialize)]
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
