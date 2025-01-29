pub struct TableStyle {
    border_collapse: BorderCollapseValue,
    caption_side: CaptionSideValue,
    empty_cells: EmptyCellsValue,
    table_layout: TableLayoutValue,
}

impl TableStyle {
    pub(crate) fn default() -> TableStyle {
        TableStyle {
            border_collapse: BorderCollapseValue::Separate,
            caption_side: CaptionSideValue::Top,
            empty_cells: EmptyCellsValue::Show,
            table_layout: TableLayoutValue::Auto,
        }
    }
}

pub enum BorderCollapseValue {
    Separate,
    Collapse,
}

pub enum CaptionSideValue {
    Top,
    Bottom,
}

pub enum EmptyCellsValue {
    Show,
    Hide,
}

pub enum TableLayoutValue {
    Auto,
    Fixed,
}
