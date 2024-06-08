use xmlserde_derives::XmlDeserialize;

use crate::r#type::Type;

#[derive(Clone, Debug, XmlDeserialize)]
pub struct Array {
    #[xmlserde(name = b"name", ty = "attr")]
    name: Option<String>,
    #[xmlserde(name = b"zero-terminated", ty = "attr")]
    zero_terminated: Option<bool>,
    #[xmlserde(name = b"fixed-size", ty = "attr")]
    fixed_size: Option<u16>,
    #[xmlserde(name = b"introspectable", ty = "attr")]
    introspectable: Option<bool>,
    #[xmlserde(name = b"length", ty = "attr")]
    length: Option<u32>,
    #[xmlserde(name = b"c:type", ty = "attr")]
    c_type: Option<String>,
    #[xmlserde(name = b"type", ty = "child")]
    type_: Type, // TODO: does this really has to be AnyType?
}

impl Array {
    pub fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    pub fn zero_terminated(&self) -> Option<bool> {
        self.zero_terminated
    }

    pub fn fixed_size(&self) -> Option<u16> {
        self.fixed_size
    }

    pub fn is_introspectable(&self) -> bool {
        self.introspectable.unwrap_or(true)
    }

    pub fn length(&self) -> Option<u32> {
        self.length
    }

    pub fn c_type(&self) -> Option<&str> {
        self.c_type.as_deref()
    }

    pub fn ty(&self) -> &Type {
        &self.type_
    }
}
