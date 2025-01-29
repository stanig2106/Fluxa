pub struct FlexStyle {
    justify_items: JustifyItemsValue,
    justify_self: JustifySelfValue,
    order: i32,
}

impl FlexStyle {
    pub(crate) fn default() -> FlexStyle {
        FlexStyle {
            justify_items: JustifyItemsValue::Normal,
            justify_self: JustifySelfValue::Auto,
            order: 0,
        }
    }
}

pub enum JustifyItemsValue {
    Normal,
    Start,
    End,
    Center,
    Stretch,
    Legacy,
}

pub enum JustifySelfValue {
    Auto,
    Start,
    End,
    Center,
    Stretch,
    Normal,
}
