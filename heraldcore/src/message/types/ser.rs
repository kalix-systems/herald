use super::*;

impl Ser for MessageSendStatus {
    fn serialize(&self, s: &mut Serializer) -> Result<S::Ok, S::Error> {
        (self as u8).ser(s)
    }
}

impl De for MessageSendStatus {
    fn deserialize(d: &mut Deserializer) -> Result<Self, D::Error> {
        use serde::de::*;
        let u = u8::de(d)?;
        u.try_into().map_err(|u| {
            E!(
                CustomError(
                    format!("expected a value between {} and {}, found {}", 0, 2, u).as_str()
                ),
                d.data.clone(),
                d.ix
            )
        })
    }
}

impl Ser for MessageReceiptStatus {
    fn serialize(&self, s: &mut Serializer) -> Result<S::Ok, S::Error> {
        (self as u8).ser(s)
    }
}

impl De for MessageReceiptStatus {
    fn deserialize(d: &mut Deserializer) -> Result<Self, D::Error> {
        use serde::de::*;
        let u = u8::de(d)?;
        u.try_into().map_err(|u| {
            E!(
                CustomError(
                    format!("expected a value between {} and {}, found {}", 0, 3, u).as_str()
                ),
                d.data.clone(),
                d.ix
            )
        })
    }
}
