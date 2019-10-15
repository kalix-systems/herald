use rust_qt_binding_generator::configuration::*;

pub struct ItemProp {
    item_property_type: SimpleType,
    optional: bool,
    roles: Vec<Vec<String>>,
    rust_by_value: bool,
    write: bool,
}

impl ItemProp {
    pub fn new(item_property_type: SimpleType) -> Self {
        Self {
            optional: false,
            rust_by_value: false,
            write: false,
            roles: vec![vec![]],
            item_property_type,
        }
    }

    pub fn optional(mut self) -> Self {
        self.optional = true;
        self
    }

    pub fn get_by_value(mut self) -> Self {
        self.rust_by_value = true;
        self
    }

    pub fn write(mut self) -> Self {
        self.write = true;
        self
    }

    pub fn build(self) -> ItemProperty {
        let ItemProp {
            item_property_type,
            rust_by_value,
            optional,
            write,
            roles,
        } = self;

        ItemProperty {
            optional,
            rust_by_value,
            write,
            item_property_type,
            roles,
        }
    }
}
