use rust_qt_binding_generator::configuration::*;

pub struct Func<'a> {
    return_type: SimpleType,
    mutable: bool,
    arguments: Vec<(&'a str, SimpleType)>,
}

impl<'a> Func<'a> {
    pub fn new(return_type: SimpleType) -> Func<'a> {
        Self {
            return_type,
            mutable: false,
            arguments: vec![],
        }
    }

    pub fn mutable(mut self) -> Self {
        self.mutable = true;
        self
    }

    pub fn arg(
        mut self,
        name: &'a str,
        typ: SimpleType,
    ) -> Self {
        self.arguments.push((name, typ));
        self
    }

    pub fn build(self) -> Function {
        let Self {
            return_type,
            mutable,
            arguments,
        } = self;

        Function {
            return_type,
            mutable,
            arguments: arguments
                .into_iter()
                .map(|(name, typ)| Argument {
                    name: name.to_owned(),
                    argument_type: typ,
                })
                .collect(),
        }
    }
}
