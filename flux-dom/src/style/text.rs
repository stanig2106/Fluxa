use crate::style::unit::UnitValue;

pub struct TextStyle {
    text_size_adjust: TextSizeAdjustValue,
    direction: DirectionValue,
    font_family: String,
    font_feature_settings: FontFeatureSettingsValue,
    font_kerning: FontKerningValue,
    font_optical_sizing: FontOpticalSizingValue,
    font_size: UnitValue,
    font_size_adjust: FontSizeAdjustValue,
    font_stretch: UnitValue,
    font_style: FontStyleValue,
    font_variant_alternates: FontVariantAlternatesValue,
    font_variant_caps: FontVariantCapsValue,
    font_variant_east_asian: FontVariantEastAsianValue,
    font_variant_emoji: FontVariantEmojiValue,
    font_variant_ligatures: FontVariantLigaturesValue,
    font_variant_numeric: FontVariantNumericValue,
    font_variant_position: FontVariantPositionValue,
    font_variation_settings: FontVariationSettingsValue,
    font_weight: FontWeightValue,
    letter_spacing: UnitValue,
    line_height: UnitValue,
    tab_size: u8,
    text_align: TextAlignValue,
    text_align_last: TextAlignLastValue,
    text_decoration_color: ColorValue,
    text_decoration_line: TextDecorationLineValue,
    text_decoration_style: TextDecorationStyleValue,
    text_decoration_thickness: UnitValue,
    text_indent: UnitValue,
    text_overflow: TextOverflowValue,
    text_shadow: TextShadowValue,
    text_transform: TextTransformValue,
    text_wrap_mode: TextWrapModeValue,
    vertical_align: VerticalAlignValue,
    white_space_collapse: WhiteSpaceCollapseValue,
    word_break: WordBreakValue,
    word_spacing: UnitValue,
}

impl TextStyle {
    pub(crate) fn default() -> TextStyle {
        TextStyle {
            text_size_adjust: TextSizeAdjustValue::Auto,
            direction: DirectionValue::Ltr,
            font_family: "sans-serif".to_string(),
            font_feature_settings: FontFeatureSettingsValue::Normal,
            font_kerning: FontKerningValue::Auto,
            font_optical_sizing: FontOpticalSizingValue::Auto,
            font_size: UnitValue::Px(16.0),
            font_size_adjust: FontSizeAdjustValue::None,
            font_stretch: UnitValue::Auto,
            font_style: FontStyleValue::Normal,
            font_variant_alternates: FontVariantAlternatesValue::Normal,
            font_variant_caps: FontVariantCapsValue::Normal,
            font_variant_east_asian: FontVariantEastAsianValue::Normal,
            font_variant_emoji: FontVariantEmojiValue::Normal,
            font_variant_ligatures: FontVariantLigaturesValue::Normal,
            font_variant_numeric: FontVariantNumericValue::Normal,
            font_variant_position: FontVariantPositionValue::Normal,
            font_variation_settings: FontVariationSettingsValue::Normal,
            font_weight: FontWeightValue::Normal,
            letter_spacing: UnitValue::Px(0.0),
            line_height: UnitValue::Auto,
            tab_size: 8,
            text_align: TextAlignValue::Start,
            text_align_last: TextAlignLastValue::Auto,
            text_decoration_color: ColorValue::Rgb(0, 0, 0),
            text_decoration_line: TextDecorationLineValue::None,
            text_decoration_style: TextDecorationStyleValue::Solid,
            text_decoration_thickness: UnitValue::Px(1.0),
            text_indent: UnitValue::Px(0.0),
            text_overflow: TextOverflowValue::Clip,
            text_shadow: TextShadowValue::None,
            text_transform: TextTransformValue::None,
            text_wrap_mode: TextWrapModeValue::Wrap,
            vertical_align: VerticalAlignValue::Baseline,
            white_space_collapse: WhiteSpaceCollapseValue::Collapse,
            word_break: WordBreakValue::Normal,
            word_spacing: UnitValue::Px(0.0),
        }
    }
}

pub enum TextSizeAdjustValue {
    Auto,
    None,
}

pub enum DirectionValue {
    Ltr,
    Rtl,
}

pub enum FontFeatureSettingsValue {
    Normal,
    Custom(String),
}

pub enum FontKerningValue {
    Auto,
    Normal,
    None,
}

pub enum FontOpticalSizingValue {
    Auto,
    None,
}

pub enum FontSizeAdjustValue {
    None,
    Custom(f32),
}

pub enum FontStyleValue {
    Normal,
    Italic,
    Oblique(f32, f32), // angle range for oblique
}

pub enum FontVariantAlternatesValue {
    Normal,
}

pub enum FontVariantCapsValue {
    Normal,
}

pub enum FontVariantEastAsianValue {
    Normal,
}

pub enum FontVariantEmojiValue {
    Normal,
}

pub enum FontVariantLigaturesValue {
    Normal,
}

pub enum FontVariantNumericValue {
    Normal,
}

pub enum FontVariantPositionValue {
    Normal,
}

pub enum FontVariationSettingsValue {
    Normal,
    Custom(String),
}

pub enum FontWeightValue {
    Normal,
    Bold,
    Weight(u16), // 100 to 900
}

pub enum TextAlignValue {
    Start,
    End,
    Center,
    Justify,
    Left,
    Right,
}

pub enum TextAlignLastValue {
    Auto,
    Start,
    End,
    Left,
    Right,
    Center,
    Justify,
}

pub enum ColorValue {
    Rgb(u8, u8, u8),
    Hex(String),
    Custom(String),
}

pub enum TextDecorationLineValue {
    None,
    Underline,
    Overline,
    LineThrough,
    Blink,
}

pub enum TextDecorationStyleValue {
    Solid,
    Double,
    Dotted,
    Dashed,
    Wavy,
}

pub enum TextOverflowValue {
    Clip,
    Ellipsis,
}

pub enum TextShadowValue {
    None,
    Custom(String), // CSS-like shadow settings
}

pub enum TextTransformValue {
    None,
    Uppercase,
    Lowercase,
    Capitalize,
}

pub enum TextWrapModeValue {
    Wrap,
    NoWrap,
    Anywhere,
}

pub enum VerticalAlignValue {
    Baseline,
    Sub,
    Super,
    TextTop,
    TextBottom,
    Middle,
    Top,
    Bottom,
}

pub enum WhiteSpaceCollapseValue {
    Collapse,
    Preserve,
}

pub enum WordBreakValue {
    Normal,
    BreakAll,
    KeepAll,
}
