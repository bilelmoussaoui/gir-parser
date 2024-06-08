use xmlserde_derives::XmlDeserialize;

#[derive(Debug, XmlDeserialize)]
pub struct Attribute {
    #[xmlserde(name = b"name", ty = "attr")]
    name: String,
    #[xmlserde(name = b"value", ty = "attr")]
    value: String,
}

impl Attribute {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn value(&self) -> &str {
        &self.value
    }
}
