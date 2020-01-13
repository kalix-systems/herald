use super::*;

impl Ser for SendStatus {
    fn ser(
        &self,
        s: &mut Serializer,
    ) {
        (*self as u8).ser(s)
    }
}

impl De for SendStatus {
    fn de(d: &mut Deserializer) -> Result<Self, KsonError> {
        let u = u8::de(d)?;

        u.try_into().map_err(|u| {
            E!(
                CustomError(format!(
                    "expected a value between {} and {}, found {}",
                    0, 1, u
                )),
                d.data.clone(),
                d.ix
            )
        })
    }
}

impl Ser for ReceiptStatus {
    fn ser(
        &self,
        s: &mut Serializer,
    ) {
        (*self as u8).ser(s)
    }
}

impl De for ReceiptStatus {
    fn de(d: &mut Deserializer) -> Result<Self, KsonError> {
        let u = u8::de(d)?;

        u.try_into().map_err(|u| {
            E!(
                CustomError(format!(
                    "expected a value between {} and {}, found {}",
                    0, 1, u
                )),
                d.data.clone(),
                d.ix
            )
        })
    }
}
