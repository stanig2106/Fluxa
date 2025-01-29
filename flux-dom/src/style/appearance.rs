use crate::style::unit::UnitValue;

pub struct AppearanceStyle {
    color: ColorValue,
    background_attachment: BackgroundAttachmentValue,
    background_clip: BackgroundClipValue,
    background_color: ColorValue,
    background_image: BackgroundImageValue,
    background_origin: BackgroundOriginValue,
    background_position_x: UnitValue,
    background_position_y: UnitValue,
    background_repeat: BackgroundRepeatValue,
    background_size: BackgroundSizeValue,
    border_bottom_color: ColorValue,
    border_bottom_style: BorderStyleValue,
    border_bottom_width: UnitValue,
    border_image_outset: UnitValue,
    border_image_repeat: BorderImageRepeatValue,
    border_image_slice: UnitValue,
    border_image_source: BorderImageSourceValue,
    border_image_width: UnitValue,
    border_left_color: ColorValue,
    border_left_style: BorderStyleValue,
    border_left_width: UnitValue,
    border_right_color: ColorValue,
    border_right_style: BorderStyleValue,
    border_right_width: UnitValue,
    border_top_color: ColorValue,
    border_top_style: BorderStyleValue,
    border_top_width: UnitValue,
    box_shadow: BoxShadowValue,
    cursor: CursorValue,
    outline_color: ColorValue,
    outline_offset: UnitValue,
    outline_style: OutlineStyleValue,
    outline_width: UnitValue,
}

impl AppearanceStyle {
    pub(crate) fn default() -> AppearanceStyle {
        AppearanceStyle {
            color: ColorValue::Rgb(0, 0, 0),
            background_attachment: BackgroundAttachmentValue::Scroll,
            background_clip: BackgroundClipValue::BorderBox,
            background_color: ColorValue::Rgb(255, 255, 255),
            background_image: BackgroundImageValue::None,
            background_origin: BackgroundOriginValue::PaddingBox,
            background_position_x: UnitValue::Auto,
            background_position_y: UnitValue::Auto,
            background_repeat: BackgroundRepeatValue::Repeat,
            background_size: BackgroundSizeValue::Auto,
            border_bottom_color: ColorValue::Rgb(0, 0, 0),
            border_bottom_style: BorderStyleValue::None,
            border_bottom_width: UnitValue::Auto,
            border_image_outset: UnitValue::Auto,
            border_image_repeat: BorderImageRepeatValue::Stretch,
            border_image_slice: UnitValue::Auto,
            border_image_source: BorderImageSourceValue::None,
            border_image_width: UnitValue::Auto,
            border_left_color: ColorValue::Rgb(0, 0, 0),
            border_left_style: BorderStyleValue::None,
            border_left_width: UnitValue::Auto,
            border_right_color: ColorValue::Rgb(0, 0, 0),
            border_right_style: BorderStyleValue::None,
            border_right_width: UnitValue::Auto,
            border_top_color: ColorValue::Rgb(0, 0, 0),
            border_top_style: BorderStyleValue::None,
            border_top_width: UnitValue::Auto,
            box_shadow: BoxShadowValue::None,
            cursor: CursorValue::Auto,
            outline_color: ColorValue::Rgb(0, 0, 0),
            outline_offset: UnitValue::Auto,
            outline_style: OutlineStyleValue::None,
            outline_width: UnitValue::Auto,
        }
    }
}

pub enum ColorValue {
    Rgb(u8, u8, u8),
    Hex(String),
    Custom(String),
}

pub enum BackgroundAttachmentValue {
    Scroll,
    Fixed,
    Local,
}

pub enum BackgroundClipValue {
    BorderBox,
    PaddingBox,
    ContentBox,
}

pub enum BackgroundImageValue {
    None,
    Url(String),
}

pub enum BackgroundOriginValue {
    BorderBox,
    PaddingBox,
    ContentBox,
}

pub enum BackgroundRepeatValue {
    Repeat,
    RepeatX,
    RepeatY,
    NoRepeat,
}

pub enum BackgroundSizeValue {
    Auto,
    Cover,
    Contain,
    Custom(String), // e.g., "50% 50%"
}

pub enum BorderStyleValue {
    None,
    Solid,
    Dotted,
    Dashed,
    Double,
    Groove,
    Ridge,
    Inset,
    Outset,
}

pub enum BorderImageRepeatValue {
    Stretch,
    Repeat,
    Round,
    Space,
}

pub enum BorderImageSourceValue {
    None,
    Url(String),
}

pub enum BoxShadowValue {
    None,
    Custom(String), // CSS-like shadow settings
}

pub enum CursorValue {
    Auto,
    Default,
    Pointer,
    Text,
    Wait,
    Move,
    NotAllowed,
    Custom(String), // For specific cursor URL
}

pub enum OutlineStyleValue {
    None,
    Solid,
    Dotted,
    Dashed,
    Double,
    Groove,
    Ridge,
    Inset,
    Outset,
}

pub enum BorderImageValue {
    None,
    Url(String),
}
