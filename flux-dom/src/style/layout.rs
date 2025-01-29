use crate::style::unit::UnitValue;

pub struct LayoutStyle {
    position: PositionValue,
    align_content: AlignContentValue,
    align_items: AlignItemsValue,
    align_self: AlignSelfValue,
    block_size: UnitValue,
    bottom: UnitValue,
    box_sizing: BoxSizingValue,
    clear: ClearValue,
    clip: UnitValue,
    display: DisplayValue,
    flex_basis: UnitValue,
    flex_direction: FlexDirectionValue,
    flex_grow: f32,
    flex_shrink: f32,
    flex_wrap: FlexWrapValue,
    float: FloatValue,
    height: UnitValue,
    inline_size: UnitValue,
    justify_content: JustifyContentValue,
    left: UnitValue,
    margin_bottom: UnitValue,
    margin_left: UnitValue,
    margin_right: UnitValue,
    margin_top: UnitValue,
    max_block_size: UnitValue,
    max_height: UnitValue,
    max_inline_size: UnitValue,
    max_width: UnitValue,
    min_block_size: UnitValue,
    min_height: UnitValue,
    min_inline_size: UnitValue,
    min_width: UnitValue,
    overflow_x: OverflowValue,
    overflow_y: OverflowValue,
    padding_bottom: UnitValue,
    padding_left: UnitValue,
    padding_right: UnitValue,
    padding_top: UnitValue,
    resize: ResizeValue,
    right: UnitValue,
    top: UnitValue,
    visibility: VisibilityValue,
    width: UnitValue,
    z_index: ZIndexValue,
}

impl LayoutStyle {
    pub(crate) fn default() -> LayoutStyle {
        LayoutStyle {
            position: PositionValue::Static,
            align_content: AlignContentValue::Normal,
            align_items: AlignItemsValue::Normal,
            align_self: AlignSelfValue::Auto,
            block_size: UnitValue::Auto,
            bottom: UnitValue::Auto,
            box_sizing: BoxSizingValue::ContentBox,
            clear: ClearValue::None,
            clip: UnitValue::Auto,
            display: DisplayValue::Block,
            flex_basis: UnitValue::Auto,
            flex_direction: FlexDirectionValue::Row,
            flex_grow: 0.0,
            flex_shrink: 1.0,
            flex_wrap: FlexWrapValue::NoWrap,
            float: FloatValue::None,
            height: UnitValue::Auto,
            inline_size: UnitValue::Auto,
            justify_content: JustifyContentValue::Normal,
            left: UnitValue::Auto,
            margin_bottom: UnitValue::Zero,
            margin_left: UnitValue::Zero,
            margin_right: UnitValue::Zero,
            margin_top: UnitValue::Zero,
            max_block_size: UnitValue::None,
            max_height: UnitValue::None,
            max_inline_size: UnitValue::None,
            max_width: UnitValue::None,
            min_block_size: UnitValue::Zero,
            min_height: UnitValue::Zero,
            min_inline_size: UnitValue::Zero,
            min_width: UnitValue::Zero,
            overflow_x: OverflowValue::Visible,
            overflow_y: OverflowValue::Visible,
            padding_bottom: UnitValue::Zero,
            padding_left: UnitValue::Zero,
            padding_right: UnitValue::Zero,
            padding_top: UnitValue::Zero,
            resize: ResizeValue::None,
            right: UnitValue::Auto,
            top: UnitValue::Auto,
            visibility: VisibilityValue::Visible,
            width: UnitValue::Auto,
            z_index: ZIndexValue::Auto,
        }
    }
}

pub enum PositionValue {
    Relative,
    Absolute,
    Fixed,
    Static,
}

pub enum AlignContentValue {
    Normal,
}

pub enum AlignItemsValue {
    Normal,
}

pub enum AlignSelfValue {
    Auto,
}

pub enum BoxSizingValue {
    ContentBox,
    BorderBox,
}

pub enum ClearValue {
    None,
    Left,
    Right,
    Both,
}

pub enum DisplayValue {
    Block,
    Inline,
    Flex,
    Grid,
    None,
}

pub enum FlexDirectionValue {
    Row,
    RowReverse,
    Column,
    ColumnReverse,
}

pub enum FlexWrapValue {
    NoWrap,
    Wrap,
    WrapReverse,
}

pub enum FloatValue {
    None,
    Left,
    Right,
}

pub enum JustifyContentValue {
    Normal,
    Start,
    End,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}

pub enum OverflowValue {
    Visible,
    Hidden,
    Scroll,
    Auto,
}

pub enum ResizeValue {
    None,
    Both,
    Horizontal,
    Vertical,
}

pub enum VisibilityValue {
    Visible,
    Hidden,
    Collapse,
}

pub enum ZIndexValue {
    Auto,
    Value(i32),
}
