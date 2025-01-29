use crate::style::unit::UnitValue;

pub struct OtherStyle {
    color_scheme: ColorSchemeValue,
    forced_color_adjust: ForcedColorAdjustValue,
    mask_image: MaskImageValue,
    math_depth: i32,
    position_anchor: PositionAnchorValue,
    appearance: AppearanceValue,
    font_palette: FontPaletteValue,
    font_synthesis_small_caps: FontSynthesisValue,
    font_synthesis_style: FontSynthesisValue,
    font_synthesis_weight: FontSynthesisValue,
    position_area: PositionAreaValue,
    text_orientation: TextOrientationValue,
    text_rendering: TextRenderingValue,
    text_spacing_trim: TextSpacingTrimValue,
    writing_mode: WritingModeValue,
    zoom: f32,
    accent_color: AccentColorValue,
    alignment_baseline: AlignmentBaselineValue,
    all: AllValue,
    anchor_name: String,
    anchor_scope: String,
    animation_composition: AnimationCompositionValue,
    animation_range_end: AnimationRangeValue,
    animation_range_start: AnimationRangeValue,
    animation_timeline: AnimationTimelineValue,
    app_region: AppRegionValue,
    aspect_ratio: AspectRatioValue,
    backdrop_filter: BackdropFilterValue,
    backface_visibility: BackfaceVisibilityValue,
    background_blend_mode: BackgroundBlendModeValue,
    baseline_shift: UnitValue,
    baseline_source: BaselineSourceValue,
    border_block_end_color: ColorValue,
    border_block_end_style: BorderStyleValue,
    border_block_end_width: UnitValue,
    border_block_start_color: ColorValue,
    border_block_start_style: BorderStyleValue,
    border_block_start_width: UnitValue,
    border_bottom_left_radius: UnitValue,
    border_bottom_right_radius: UnitValue,
    border_end_end_radius: UnitValue,
    border_end_start_radius: UnitValue,
    border_inline_end_color: ColorValue,
    border_inline_end_style: BorderStyleValue,
    border_inline_end_width: UnitValue,
    border_inline_start_color: ColorValue,
    border_inline_start_style: BorderStyleValue,
    border_inline_start_width: UnitValue,
    border_start_end_radius: UnitValue,
    border_start_start_radius: UnitValue,
    border_top_left_radius: UnitValue,
    border_top_right_radius: UnitValue,
    box_decoration_break: BoxDecorationBreakValue,
    break_after: BreakValue,
    break_before: BreakValue,
    break_inside: BreakValue,
    buffered_rendering: BufferedRenderingValue,
    caret_color: ColorValue,
    clip_path: ClipPathValue,
    clip_rule: ClipRuleValue,
    color_interpolation: ColorInterpolationValue,
    color_interpolation_filters: ColorInterpolationFiltersValue,
    color_rendering: ColorRenderingValue,
    column_count: ColumnCountValue,
    column_fill: ColumnFillValue,
    column_gap: ColumnGapValue,
    column_rule_color: ColorValue,
    column_rule_style: BorderStyleValue,
    column_rule_width: UnitValue,
    column_span: ColumnSpanValue,
    column_width: UnitValue,
    contain: ContainValue,
    contain_intrinsic_block_size: ContainIntrinsicSizeValue,
    contain_intrinsic_height: ContainIntrinsicSizeValue,
    contain_intrinsic_inline_size: ContainIntrinsicSizeValue,
    contain_intrinsic_width: ContainIntrinsicSizeValue,
    container_name: String,
    container_type: ContainerTypeValue,
    content_visibility: ContentVisibilityValue,
    counter_set: CounterSetValue,
    cx: UnitValue,
    cy: UnitValue,
    d: DValue,
    dominant_baseline: DominantBaselineValue,
    field_sizing: FieldSizingValue,
    fill: ColorValue,
    fill_opacity: f32,
    fill_rule: FillRuleValue,
    filter: FilterValue,
    flood_color: ColorValue,
    flood_opacity: f32,
    hyphenate_character: String,
    hyphenate_limit_chars: HyphenateLimitCharsValue,
    hyphens: HyphensValue,
    image_orientation: ImageOrientationValue,
    image_rendering: ImageRenderingValue,
    initial_letter: InitialLetterValue,
    inset_block_end: UnitValue,
    inset_block_start: UnitValue,
    inset_inline_end: UnitValue,
    inset_inline_start: UnitValue,
    interpolate_size: InterpolateSizeValue,
    isolation: IsolationValue,
    lighting_color: ColorValue,
    line_break: LineBreakValue,
    list_style_image: ListStyleImageValue,
    list_style_position: ListStylePositionValue,
    list_style_type: ListStyleTypeValue,
    margin_block_end: UnitValue,
    margin_block_start: UnitValue,
    margin_inline_end: UnitValue,
    margin_inline_start: UnitValue,
    marker_end: MarkerValue,
    marker_mid: MarkerValue,
    marker_start: MarkerValue,
    mask_clip: MaskClipValue,
    mask_composite: MaskCompositeValue,
    mask_mode: MaskModeValue,
    mask_origin: MaskOriginValue,
    mask_repeat: MaskRepeatValue,
    mask_size: MaskSizeValue,
    mask_type: MaskTypeValue,
    math_shift: MathShiftValue,
    math_style: MathStyleValue,
    mix_blend_mode: MixBlendModeValue,
    object_fit: ObjectFitValue,
    object_position: ObjectPositionValue,
    object_view_box: ObjectViewBoxValue,
    offset_anchor: OffsetAnchorValue,
    offset_distance: UnitValue,
    offset_path: OffsetPathValue,
    offset_position: OffsetPositionValue,
    offset_rotate: OffsetRotateValue,
    opacity: f32,
    orphans: u8,
    overflow_anchor: OverflowAnchorValue,
    overflow_clip_margin: UnitValue,
    overflow_wrap: OverflowWrapValue,
    overlay: OverlayValue,
    overscroll_behavior_block: OverscrollBehaviorValue,
    overscroll_behavior_inline: OverscrollBehaviorValue,
    overscroll_behavior_x: OverscrollBehaviorValue,
    overscroll_behavior_y: OverscrollBehaviorValue,
    padding_block_end: UnitValue,
    padding_block_start: UnitValue,
    padding_inline_end: UnitValue,
    padding_inline_start: UnitValue,
    page: PageValue,
    page_orientation: PageOrientationValue,
    paint_order: PaintOrderValue,
    perspective: UnitValue,
    perspective_origin: PerspectiveOriginValue,
    pointer_events: PointerEventsValue,
    position_try_fallbacks: PositionTryFallbacksValue,
    position_try_order: PositionTryOrderValue,
    position_visibility: PositionVisibilityValue,
    r: UnitValue,
    rotate: RotateValue,
    row_gap: RowGapValue,
    ruby_align: RubyAlignValue,
    ruby_position: RubyPositionValue,
    rx: UnitValue,
    ry: UnitValue,
    scale: ScaleValue,
    scroll_behavior: ScrollBehaviorValue,
    scroll_margin_block_end: UnitValue,
    scroll_margin_block_start: UnitValue,
    scroll_margin_bottom: UnitValue,
    scroll_margin_inline_end: UnitValue,
    scroll_margin_inline_start: UnitValue,
    scroll_margin_left: UnitValue,
    scroll_margin_right: UnitValue,
    scroll_margin_top: UnitValue,
    scroll_padding_block_end: UnitValue,
    scroll_padding_block_start: UnitValue,
    scroll_padding_bottom: UnitValue,
    scroll_padding_inline_end: UnitValue,
    scroll_padding_inline_start: UnitValue,
    scroll_padding_left: UnitValue,
    scroll_padding_right: UnitValue,
    scroll_padding_top: UnitValue,
    scroll_snap_align: ScrollSnapAlignValue,
    scroll_snap_stop: ScrollSnapStopValue,
    scroll_snap_type: ScrollSnapTypeValue,
    scroll_timeline_axis: ScrollTimelineAxisValue,
    scroll_timeline_name: String,
    scrollbar_color: ScrollbarColorValue,
    scrollbar_gutter: ScrollbarGutterValue,
    scrollbar_width: ScrollbarWidthValue,
    shape_image_threshold: f32,
    shape_margin: UnitValue,
    shape_outside: ShapeOutsideValue,
    shape_rendering: ShapeRenderingValue,
    size: SizeValue,
    speak: SpeakValue,
    stop_color: ColorValue,
    stop_opacity: f32,
    stroke: StrokeValue,
    stroke_dasharray: StrokeDashArrayValue,
    stroke_dashoffset: UnitValue,
    stroke_linecap: StrokeLineCapValue,
    stroke_linejoin: StrokeLineJoinValue,
    stroke_miterlimit: f32,
    stroke_opacity: f32,
    stroke_width: UnitValue,
    text_anchor: TextAnchorValue,
    text_combine_upright: TextCombineUprightValue,
    text_decoration_skip_ink: TextDecorationSkipInkValue,
    text_emphasis_color: ColorValue,
    text_emphasis_position: TextEmphasisPositionValue,
    text_emphasis_style: TextEmphasisStyleValue,
    text_underline_offset: UnitValue,
    text_underline_position: TextUnderlinePositionValue,
    text_wrap_style: TextWrapStyleValue,
    timeline_scope: TimelineScopeValue,
    touch_action: TouchActionValue,
    transform: TransformValue,
    transform_box: TransformBoxValue,
    transform_origin: TransformOriginValue,
    transform_style: TransformStyleValue,
    translate: TranslateValue,
    unicode_bidi: UnicodeBidiValue,
    user_select: UserSelectValue,
    vector_effect: VectorEffectValue,
    view_timeline_axis: ViewTimelineAxisValue,
    view_timeline_inset: ViewTimelineInsetValue,
    view_timeline_name: ViewTimelineNameValue,
    view_transition_class: ViewTransitionClassValue,
    view_transition_name: ViewTransitionNameValue,
    widows: u8,
    will_change: WillChangeValue,
    x: UnitValue,
    y: UnitValue,
}

impl OtherStyle {
    pub(crate) fn default() -> OtherStyle {
        OtherStyle {
            color_scheme: ColorSchemeValue::Auto,
            forced_color_adjust: ForcedColorAdjustValue::Auto,
            mask_image: MaskImageValue::None,
            math_depth: 0,
            position_anchor: PositionAnchorValue::Auto,
            appearance: AppearanceValue::None,
            font_palette: FontPaletteValue::Normal,
            font_synthesis_small_caps: FontSynthesisValue::Auto,
            font_synthesis_style: FontSynthesisValue::Auto,
            font_synthesis_weight: FontSynthesisValue::Auto,
            position_area: PositionAreaValue::None,
            text_orientation: TextOrientationValue::Mixed,
            text_rendering: TextRenderingValue::Auto,
            text_spacing_trim: TextSpacingTrimValue::None,
            writing_mode: WritingModeValue::HorizontalTb,
            zoom: 1.0,
            accent_color: AccentColorValue::Auto,
            alignment_baseline: AlignmentBaselineValue::Auto,
            all: AllValue::Initial,
            anchor_name: "".to_string(),
            anchor_scope: "".to_string(),
            animation_composition: AnimationCompositionValue::Replace,
            animation_range_end: AnimationRangeValue::Normal,
            animation_range_start: AnimationRangeValue::Normal,
            animation_timeline: AnimationTimelineValue::Auto,
            app_region: AppRegionValue::None,
            aspect_ratio: AspectRatioValue::Auto,
            backdrop_filter: BackdropFilterValue::None,
            backface_visibility: BackfaceVisibilityValue::Visible,
            background_blend_mode: BackgroundBlendModeValue::Normal,
            baseline_shift: UnitValue::Auto,
            baseline_source: BaselineSourceValue::Auto,
            border_block_end_color: ColorValue::Rgb(0, 0, 0),
            border_block_end_style: BorderStyleValue::None,
            border_block_end_width: UnitValue::Auto,
            border_block_start_color: ColorValue::Rgb(0, 0, 0),
            border_block_start_style: BorderStyleValue::None,
            border_block_start_width: UnitValue::Auto,
            border_bottom_left_radius: UnitValue::Auto,
            border_bottom_right_radius: UnitValue::Auto,
            border_end_end_radius: UnitValue::Auto,
            border_end_start_radius: UnitValue::Auto,
            border_inline_end_color: ColorValue::Rgb(0, 0, 0),
            border_inline_end_style: BorderStyleValue::None,
            border_inline_end_width: UnitValue::Auto,
            border_inline_start_color: ColorValue::Rgb(0, 0, 0),
            border_inline_start_style: BorderStyleValue::None,
            border_inline_start_width: UnitValue::Zero,
            border_start_end_radius: UnitValue::Zero,
            border_start_start_radius: UnitValue::Zero,
            border_top_left_radius: UnitValue::Zero,
            border_top_right_radius: UnitValue::Zero,
            box_decoration_break: BoxDecorationBreakValue::Slice,
            break_after: BreakValue::Auto,
            break_before: BreakValue::Auto,
            break_inside: BreakValue::Auto,
            buffered_rendering: BufferedRenderingValue::Auto,
            caret_color: ColorValue::Rgb(0, 0, 0),
            clip_path: ClipPathValue::None,
            clip_rule: ClipRuleValue::NonZero,
            color_interpolation: ColorInterpolationValue::Auto,
            color_interpolation_filters: ColorInterpolationFiltersValue::Auto,
            color_rendering: ColorRenderingValue::Auto,
            column_count: ColumnCountValue::Auto,
            column_fill: ColumnFillValue::Auto,
            column_gap: ColumnGapValue::Normal,
            column_rule_color: ColorValue::Rgb(0, 0, 0),
            column_rule_style: BorderStyleValue::None,
            column_rule_width: UnitValue::Zero,
            column_span: ColumnSpanValue::None,
            column_width: UnitValue::Zero,
            contain: ContainValue::None,
            contain_intrinsic_block_size: ContainIntrinsicSizeValue::None,
            contain_intrinsic_height: ContainIntrinsicSizeValue::None,
            contain_intrinsic_inline_size: ContainIntrinsicSizeValue::None,
            contain_intrinsic_width: ContainIntrinsicSizeValue::None,
            container_name: "".to_string(),
            container_type: ContainerTypeValue::Normal,
            content_visibility: ContentVisibilityValue::Visible,
            counter_set: CounterSetValue::None,
            cx: UnitValue::Zero,
            cy: UnitValue::Zero,
            d: DValue::None,
            dominant_baseline: DominantBaselineValue::Auto,
            field_sizing: FieldSizingValue::Fixed,
            fill: ColorValue::Rgb(0, 0, 0),
            fill_opacity: 0.0,
            fill_rule: FillRuleValue::NonZero,
            filter: FilterValue::None,
            flood_color: ColorValue::Rgb(0, 0, 0),
            flood_opacity: 0.0,
            hyphenate_character: "".to_string(),
            hyphenate_limit_chars: HyphenateLimitCharsValue::Auto,
            hyphens: HyphensValue::None,
            image_orientation: ImageOrientationValue::Auto,
            image_rendering: ImageRenderingValue::Auto,
            initial_letter: InitialLetterValue::Normal,
            inset_block_end: UnitValue::Zero,
            inset_block_start: UnitValue::Zero,
            inset_inline_end: UnitValue::Zero,
            inset_inline_start: UnitValue::Zero,
            interpolate_size: InterpolateSizeValue::Auto,
            isolation: IsolationValue::Auto,
            lighting_color: ColorValue::Rgb(255, 255, 255),
            line_break: LineBreakValue::Auto,
            list_style_image: ListStyleImageValue::None,
            list_style_position: ListStylePositionValue::Inside,
            list_style_type: ListStyleTypeValue::None,
            margin_block_end: UnitValue::Zero,
            margin_block_start: UnitValue::Zero,
            margin_inline_end: UnitValue::Zero,
            margin_inline_start: UnitValue::Zero,
            marker_end: MarkerValue::None,
            marker_mid: MarkerValue::None,
            marker_start: MarkerValue::None,
            mask_clip: MaskClipValue::BorderBox,
            mask_composite: MaskCompositeValue::Add,
            mask_mode: MaskModeValue::MatchSource,
            mask_origin: MaskOriginValue::BorderBox,
            mask_repeat: MaskRepeatValue::Repeat,
            mask_size: MaskSizeValue::Auto,
            mask_type: MaskTypeValue::Luminance,
            math_shift: MathShiftValue::Normal,
            math_style: MathStyleValue::Normal,
            mix_blend_mode: MixBlendModeValue::Normal,
            object_fit: ObjectFitValue::Fill,
            object_position: ObjectPositionValue::Percentage(50.0, 50.0),
            object_view_box: ObjectViewBoxValue::None,
            offset_anchor: OffsetAnchorValue::Auto,
            offset_distance: UnitValue::Zero,
            offset_path: OffsetPathValue::None,
            offset_position: OffsetPositionValue::Normal,
            offset_rotate: OffsetRotateValue::None,
            opacity: 0.0,
            orphans: 0,
            overflow_anchor: OverflowAnchorValue::Auto,
            overflow_clip_margin: UnitValue::Zero,
            overflow_wrap: OverflowWrapValue::Normal,
            overlay: OverlayValue::None,
            overscroll_behavior_block: OverscrollBehaviorValue::Auto,
            overscroll_behavior_inline: OverscrollBehaviorValue::Auto,
            overscroll_behavior_x: OverscrollBehaviorValue::Auto,
            overscroll_behavior_y: OverscrollBehaviorValue::Auto,
            padding_block_end: UnitValue::Zero,
            padding_block_start: UnitValue::Zero,
            padding_inline_end: UnitValue::Zero,
            padding_inline_start: UnitValue::Zero,
            page: PageValue::Auto,
            page_orientation: PageOrientationValue::Portrait,
            paint_order: PaintOrderValue::Normal,
            perspective: UnitValue::Zero,
            perspective_origin: PerspectiveOriginValue::Custom(UnitValue::Zero, UnitValue::Zero),
            pointer_events: PointerEventsValue::Auto,
            position_try_fallbacks: PositionTryFallbacksValue::None,
            position_try_order: PositionTryOrderValue::Normal,
            position_visibility: PositionVisibilityValue::Always,
            r: UnitValue::Zero,
            rotate: RotateValue::None,
            row_gap: RowGapValue::Normal,
            ruby_align: RubyAlignValue::SpaceAround,
            ruby_position: RubyPositionValue::Over,
            rx: UnitValue::Zero,
            ry: UnitValue::Zero,
            scale: ScaleValue::None,
            scroll_behavior: ScrollBehaviorValue::Auto,
            scroll_margin_block_end: UnitValue::Zero,
            scroll_margin_block_start: UnitValue::Zero,
            scroll_margin_bottom: UnitValue::Zero,
            scroll_margin_inline_end: UnitValue::Zero,
            scroll_margin_inline_start: UnitValue::Zero,
            scroll_margin_left: UnitValue::Zero,
            scroll_margin_right: UnitValue::Zero,
            scroll_margin_top: UnitValue::Zero,
            scroll_padding_block_end: UnitValue::Zero,
            scroll_padding_block_start: UnitValue::Zero,
            scroll_padding_bottom: UnitValue::Zero,
            scroll_padding_inline_end: UnitValue::Zero,
            scroll_padding_inline_start: UnitValue::Zero,
            scroll_padding_left: UnitValue::Zero,
            scroll_padding_right: UnitValue::Zero,
            scroll_padding_top: UnitValue::Zero,
            scroll_snap_align: ScrollSnapAlignValue::None,
            scroll_snap_stop: ScrollSnapStopValue::Normal,
            scroll_snap_type: ScrollSnapTypeValue::None,
            scroll_timeline_axis: ScrollTimelineAxisValue::Block,
            scroll_timeline_name: "".to_string(),
            scrollbar_color: ScrollbarColorValue::Auto,
            scrollbar_gutter: ScrollbarGutterValue::Auto,
            scrollbar_width: ScrollbarWidthValue::Auto,
            shape_image_threshold: 0.0,
            shape_margin: UnitValue::Zero,
            shape_outside: ShapeOutsideValue::None,
            shape_rendering: ShapeRenderingValue::Auto,
            size: SizeValue::Auto,
            speak: SpeakValue::Normal,
            stop_color: ColorValue::Rgb(0, 0, 0),
            stop_opacity: 0.0,
            stroke: StrokeValue::None,
            stroke_dasharray: StrokeDashArrayValue::None,
            stroke_dashoffset: UnitValue::Zero,
            stroke_linecap: StrokeLineCapValue::Butt,
            stroke_linejoin: StrokeLineJoinValue::Miter,
            stroke_miterlimit: 0.0,
            stroke_opacity: 0.0,
            stroke_width: UnitValue::Zero,
            text_anchor: TextAnchorValue::Start,
            text_combine_upright: TextCombineUprightValue::None,
            text_decoration_skip_ink: TextDecorationSkipInkValue::Auto,
            text_emphasis_color: ColorValue::Rgb(0, 0, 0),
            text_emphasis_position: TextEmphasisPositionValue::Over,
            text_emphasis_style: TextEmphasisStyleValue::None,
            text_underline_offset: UnitValue::Zero,
            text_underline_position: TextUnderlinePositionValue::Auto,
            text_wrap_style: TextWrapStyleValue::Auto,
            timeline_scope: TimelineScopeValue::None,
            touch_action: TouchActionValue::Auto,
            transform: TransformValue::None,
            transform_box: TransformBoxValue::ViewBox,
            transform_origin: TransformOriginValue::Custom(UnitValue::Zero, UnitValue::Zero),
            transform_style: TransformStyleValue::Flat,
            translate: TranslateValue::None,
            unicode_bidi: UnicodeBidiValue::Normal,
            user_select: UserSelectValue::Auto,
            vector_effect: VectorEffectValue::None,
            view_timeline_axis: ViewTimelineAxisValue::Block,
            view_timeline_inset: ViewTimelineInsetValue::Auto,
            view_timeline_name: ViewTimelineNameValue::None,
            view_transition_class: ViewTransitionClassValue::None,
            view_transition_name: ViewTransitionNameValue::None,
            widows: 0,
            will_change: WillChangeValue::Auto,
            x: UnitValue::Zero,
            y: UnitValue::Zero,
        }
    }
}

pub enum ColorSchemeValue {
    Light,
    Dark,
    Auto,
}

pub enum ForcedColorAdjustValue {
    Auto,
    None,
}

pub enum MaskImageValue {
    None,
    Url(String),
}

pub enum PositionAnchorValue {
    Auto,
    Custom(String),
}

pub enum AppearanceValue {
    None,
    Auto,
}

pub enum FontPaletteValue {
    Normal,
    Custom(String),
}

pub enum FontSynthesisValue {
    Auto,
    None,
}

pub enum PositionAreaValue {
    None,
    Custom(String),
}

pub enum TextOrientationValue {
    Mixed,
    Upright,
    Sideways,
}

pub enum TextRenderingValue {
    Auto,
    OptimizeSpeed,
    OptimizeLegibility,
    GeometricPrecision,
}

pub enum TextSpacingTrimValue {
    None,
    Normal,
}

pub enum WritingModeValue {
    HorizontalTb,
    VerticalRl,
    VerticalLr,
}

pub enum AccentColorValue {
    Auto,
    Color(String),
}

pub enum AlignmentBaselineValue {
    Auto,
    Baseline,
    Center,
    TextBeforeEdge,
    TextAfterEdge,
    Middle,
    Top,
    Bottom,
    Ideographic,
    Alphabetic,
    Hanging,
    Mathematical,
}

pub enum AllValue {
    Initial,
    Inherit,
    Revert,
    Unset,
}

pub enum AnimationCompositionValue {
    Replace,
    Add,
    Accumulate,
}

pub enum AnimationRangeValue {
    Normal,
    Custom(String),
}

pub enum AnimationTimelineValue {
    Auto,
    None,
    Custom(String),
}

pub enum AppRegionValue {
    None,
    Drag,
    NoDrag,
}

pub enum AspectRatioValue {
    Auto,
    Ratio(f32), // e.g., 16:9 as 1.7777
}

pub enum BackdropFilterValue {
    None,
    Blur(f32),
    Custom(String),
}

pub enum BackfaceVisibilityValue {
    Visible,
    Hidden,
}

pub enum BackgroundBlendModeValue {
    Normal,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
    ColorDodge,
    ColorBurn,
    HardLight,
    SoftLight,
    Difference,
    Exclusion,
    Hue,
    Saturation,
    Color,
    Luminosity,
}

pub enum BoxDecorationBreakValue {
    Slice,
    Clone,
}

pub enum BreakValue {
    Auto,
    Avoid,
    Always,
    All,
    AvoidPage,
    Page,
    Left,
    Right,
    Column,
}

pub enum BufferedRenderingValue {
    Auto,
    Static,
    Dynamic,
}

pub enum ClipPathValue {
    None,
    Url(String),
    Shape(String),
}

pub enum ClipRuleValue {
    NonZero,
    EvenOdd,
}

pub enum ColorInterpolationValue {
    Auto,
    SRgb,
    LinearRgb,
}

pub enum ColorInterpolationFiltersValue {
    Auto,
    SRgb,
    LinearRgb,
}

pub enum ColorRenderingValue {
    Auto,
    OptimizeSpeed,
    OptimizeQuality,
}

pub enum ColumnCountValue {
    Auto,
    Custom(u32),
}

pub enum ColumnFillValue {
    Auto,
    Balance,
    BalanceAll,
}

pub enum ColumnGapValue {
    Normal,
    Custom(UnitValue),
}

pub enum ColumnSpanValue {
    None,
    All,
}

pub enum ContainValue {
    None,
    Strict,
    Content,
    Style,
    Size,
    Layout,
    Paint,
}

pub enum ContainIntrinsicSizeValue {
    None,
    Custom(UnitValue),
}

pub enum ContainerTypeValue {
    Normal,
    InlineSize,
    BlockSize,
}

pub enum ContentVisibilityValue {
    Visible,
    Auto,
    Hidden,
}

pub enum CounterSetValue {
    None,
    Custom(String),
}

pub enum DValue {
    None,
    Path(String),
}

pub enum DominantBaselineValue {
    Auto,
    TextBeforeEdge,
    TextAfterEdge,
    Central,
    Middle,
    Ideographic,
    Alphabetic,
    Hanging,
    Mathematical,
    Baseline,
}

pub enum FieldSizingValue {
    Fixed,
    Auto,
}

pub enum FillRuleValue {
    NonZero,
    EvenOdd,
}

pub enum FilterValue {
    None,
    Blur(f32),
    Brightness(f32),
    Contrast(f32),
    DropShadow(String),
    Grayscale(f32),
    HueRotate(f32),
    Invert(f32),
    Opacity(f32),
    Saturate(f32),
    Sepia(f32),
    Custom(String),
}

pub enum HyphensValue {
    None,
    Manual,
    Auto,
}

pub enum ImageOrientationValue {
    Auto,
    FromImage,
}

pub enum ImageRenderingValue {
    Auto,
    CrispEdges,
    Pixelated,
}

pub enum InitialLetterValue {
    Normal,
    Custom(f32),
}

pub enum IsolationValue {
    Auto,
    Isolate,
}

pub enum LineBreakValue {
    Auto,
    Loose,
    Normal,
    Strict,
}

pub enum ListStyleImageValue {
    None,
    Url(String),
}

pub enum ListStylePositionValue {
    Inside,
    Outside,
}

pub enum ListStyleTypeValue {
    None,
    Disc,
    Circle,
    Square,
    Decimal,
    DecimalLeadingZero,
    LowerRoman,
    UpperRoman,
    LowerGreek,
    UpperGreek,
    Armenian,
    Georgian,
    LowerAlpha,
    UpperAlpha,
    Custom(String),
}

pub enum MixBlendModeValue {
    Normal,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
    ColorDodge,
    ColorBurn,
    HardLight,
    SoftLight,
    Difference,
    Exclusion,
    Hue,
    Saturation,
    Color,
    Luminosity,
}

pub enum ObjectFitValue {
    Fill,
    Contain,
    Cover,
    None,
    ScaleDown,
}

pub enum OverscrollBehaviorValue {
    Auto,
    Contain,
    None,
}

pub enum PointerEventsValue {
    Auto,
    None,
    VisiblePainted,
    VisibleFill,
    VisibleStroke,
    Visible,
    Painted,
    Fill,
    Stroke,
    All,
}

pub enum RubyAlignValue {
    SpaceAround,
    Start,
    Center,
    SpaceBetween,
}

pub enum RubyPositionValue {
    Over,
    Under,
    InterCharacter,
}

pub enum ScrollBehaviorValue {
    Auto,
    Smooth,
}

pub enum ScrollSnapAlignValue {
    None,
    Start,
    End,
    Center,
}

pub enum ScrollSnapStopValue {
    Normal,
    Always,
}

pub enum ScrollSnapTypeValue {
    None,
    X,
    Y,
    Block,
    Inline,
    Both,
}

pub enum ShapeRenderingValue {
    Auto,
    OptimizeSpeed,
    CrispEdges,
    GeometricPrecision,
}

pub enum TextDecorationSkipInkValue {
    Auto,
    None,
    All,
}

pub enum TextEmphasisPositionValue {
    Over,
    Under,
    Left,
    Right,
}

pub enum TextEmphasisStyleValue {
    None,
    Dot,
    Circle,
    DoubleCircle,
    Triangle,
    Filled,
    Open,
    Custom(String),
}

pub enum TextUnderlinePositionValue {
    Auto,
    Under,
    Left,
    Right,
}

pub enum TransformValue {
    None,
    Matrix(f32, f32, f32, f32, f32, f32),
    Matrix3d([f32; 16]),
    Rotate(f32),
    Scale(f32),
    Translate(UnitValue, UnitValue),
    Skew(f32, f32),
}

pub enum UnicodeBidiValue {
    Normal,
    Embed,
    BidiOverride,
    Isolate,
    IsolateOverride,
    Plaintext,
}

pub enum UserSelectValue {
    Auto,
    None,
    Text,
    All,
    Contain,
}

pub enum VectorEffectValue {
    None,
    NonScalingStroke,
    Custom(String),
}

pub enum BaselineSourceValue {
    Auto,
    Alphabetic,
    Ideographic,
    Custom(String),
}

pub enum ColorValue {
    Rgb(u8, u8, u8),
    Hex(String),
    Custom(String),
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

pub enum HyphenateLimitCharsValue {
    Auto,
    Custom { min: u8, max: u8 },
}

pub enum InterpolateSizeValue {
    Auto,
    NumericOnly,
}

pub enum MarkerValue {
    None,
    Url(String),
}

pub enum MaskClipValue {
    BorderBox,
    PaddingBox,
    ContentBox,
    Text,
}

pub enum MaskCompositeValue {
    Add,
    Subtract,
    Intersect,
    Exclude,
}

pub enum MaskModeValue {
    MatchSource,
    Luminance,
    Alpha,
}

pub enum MaskOriginValue {
    BorderBox,
    PaddingBox,
    ContentBox,
}

pub enum MaskRepeatValue {
    Repeat,
    RepeatX,
    RepeatY,
    NoRepeat,
}

pub enum MaskSizeValue {
    Auto,
    Cover,
    Contain,
    Custom(String),
}

pub enum MaskTypeValue {
    Luminance,
    Alpha,
}

pub enum MathShiftValue {
    Normal,
    Subscript,
    Superscript,
}

pub enum MathStyleValue {
    Normal,
    Compact,
}

pub enum ObjectPositionValue {
    Percentage(f32, f32),
    Custom(String),
}

pub enum ObjectViewBoxValue {
    None,
    Custom(String),
}

pub enum OffsetAnchorValue {
    Auto,
    Custom(String),
}

pub enum OffsetPathValue {
    None,
    Path(String),
}

pub enum OffsetPositionValue {
    Normal,
    Custom(String),
}

pub enum OffsetRotateValue {
    Auto(f32), // e.g., auto with rotation angle
    None,
    Custom(f32),
}

pub enum OverflowAnchorValue {
    Auto,
    None,
}

pub enum OverflowWrapValue {
    Normal,
    BreakWord,
}

pub enum OverlayValue {
    None,
    Custom(String),
}

pub enum PageValue {
    Auto,
    Custom(String),
}

pub enum PageOrientationValue {
    Portrait,
    Landscape,
}

pub enum PaintOrderValue {
    Normal,
    FillStroke,
    StrokeFill,
}

pub enum PerspectiveOriginValue {
    Custom(UnitValue, UnitValue),
}

pub enum PositionTryFallbacksValue {
    None,
    Custom(String),
}

pub enum PositionVisibilityValue {
    Always,
    Custom(String),
}

pub enum RotateValue {
    None,
    Degrees(f32),
}

pub enum RowGapValue {
    Normal,
    Custom(UnitValue),
}

pub enum ScaleValue {
    None,
    Uniform(f32),
    Custom(f32, f32),
}

pub enum ScrollTimelineAxisValue {
    Block,
    Inline,
}

pub enum ScrollbarColorValue {
    Auto,
    Custom(ColorValue, ColorValue),
}

pub enum ScrollbarGutterValue {
    Auto,
    Stable,
    Both,
}

pub enum ScrollbarWidthValue {
    Auto,
    Thin,
    None,
}

pub enum ShapeOutsideValue {
    None,
    Url(String),
    Custom(String),
}

pub enum SizeValue {
    Auto,
    Custom(UnitValue),
}

pub enum SpeakValue {
    Normal,
    None,
}

pub enum StrokeValue {
    None,
    Color(ColorValue),
}

pub enum StrokeDashArrayValue {
    None,
    Custom(Vec<UnitValue>),
}

pub enum StrokeLineCapValue {
    Butt,
    Round,
    Square,
}

pub enum StrokeLineJoinValue {
    Miter,
    Round,
    Bevel,
}

pub enum TextAnchorValue {
    Start,
    Middle,
    End,
}

pub enum TextCombineUprightValue {
    None,
    All,
    Digits(u8),
}

pub enum TextWrapStyleValue {
    Auto,
    Custom(String),
}

pub enum TimelineScopeValue {
    None,
    Custom(String),
}

pub enum TouchActionValue {
    Auto,
    None,
    PanX,
    PanY,
    PinchZoom,
}

pub enum TransformBoxValue {
    ViewBox,
    BorderBox,
    FillBox,
    StrokeBox,
    ContentBox,
}

pub enum TransformOriginValue {
    Custom(UnitValue, UnitValue),
}

pub enum TransformStyleValue {
    Flat,
    Preserve3D,
}

pub enum TranslateValue {
    None,
    Custom(UnitValue, UnitValue),
}

pub enum ViewTimelineAxisValue {
    Block,
    Inline,
}

pub enum ViewTimelineInsetValue {
    Auto,
    Custom(String),
}

pub enum ViewTimelineNameValue {
    None,
    Custom(String),
}

pub enum ViewTransitionClassValue {
    None,
    Custom(String),
}

pub enum ViewTransitionNameValue {
    None,
    Custom(String),
}

pub enum TextCombineValue {
    None,
    All,
}

pub enum TextSecurityValue {
    None,
    Circle,
    Disc,
    Square,
}

pub enum WillChangeValue {
    Auto,
    Custom(String),
}

pub enum PositionTryOrderValue {
    Normal,
    Custom(String),
}
