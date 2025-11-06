use xmlserde_derives::XmlDeserialize;

#[derive(Clone, Debug, PartialEq, Eq, Hash, XmlDeserialize)]
#[xmlserde(root = b"attribute")]
#[xmlserde(deny_unknown_fields)]
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
