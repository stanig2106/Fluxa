use crate::style::appearance::AppearanceStyle;
use crate::style::flex::FlexStyle;
use crate::style::grid::GridStyle;
use crate::style::layout::LayoutStyle;
use crate::style::other::OtherStyle;
use crate::style::table::TableStyle;
use crate::style::text::TextStyle;

mod layout;
mod unit;
mod text;
mod appearance;
mod flex;
mod grid;
mod table;
mod other;

pub struct Style {
    layout: LayoutStyle,
    appearance: AppearanceStyle,
    flex: FlexStyle,
    grid: GridStyle,
    table: TableStyle,
    text: TextStyle,
    other: OtherStyle,
}

impl Style {
    pub fn default() {
        Style {
            layout: LayoutStyle::default(),
            appearance: AppearanceStyle::default(),
            flex: FlexStyle::default(),
            grid: GridStyle::default(),
            table: TableStyle::default(),
            text: TextStyle::default(),
            other: OtherStyle::default(),
        };
    }
}
