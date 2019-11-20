use super::*;

impl Ser for MessageSendStatus {
    fn ser(
        &self,
        s: &mut Serializer,
    ) {
        (*self as u8).ser(s)
    }
}

impl De for MessageSendStatus {
    fn de(d: &mut Deserializer) -> Result<Self, KsonError> {
        let u = u8::de(d)?;

        u.try_into().map_err(|u| {
            E!(
                CustomError(format!(
                    "expected a value between {} and {}, found {}",
                    0, 2, u
                )),
                d.data.clone(),
                d.ix
            )
        })
    }
}

impl Ser for MessageReceiptStatus {
    fn ser(
        &self,
        s: &mut Serializer,
    ) {
        (*self as u8).ser(s)
    }
}

impl De for MessageReceiptStatus {
    fn de(d: &mut Deserializer) -> Result<Self, KsonError> {
        let u = u8::de(d)?;

        u.try_into().map_err(|u| {
            E!(
                CustomError(format!(
                    "expected a value between {} and {}, found {}",
                    0, 3, u
                )),
                d.data.clone(),
                d.ix
            )
        })
    }
}
