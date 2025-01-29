use crate::style::unit::UnitValue;

pub struct GridStyle {
    grid_auto_columns: GridAutoSizeValue,
    grid_auto_flow: GridAutoFlowValue,
    grid_auto_rows: GridAutoSizeValue,
    grid_column_end: GridLineValue,
    grid_column_start: GridLineValue,
    grid_row_end: GridLineValue,
    grid_row_start: GridLineValue,
    grid_template_areas: GridTemplateAreasValue,
    grid_template_columns: GridTemplateValue,
    grid_template_rows: GridTemplateValue,
}

impl GridStyle {
    pub(crate) fn default() -> GridStyle {
        GridStyle {
            grid_auto_columns: GridAutoSizeValue::Auto,
            grid_auto_flow: GridAutoFlowValue::Row,
            grid_auto_rows: GridAutoSizeValue::Auto,
            grid_column_end: GridLineValue::Auto,
            grid_column_start: GridLineValue::Auto,
            grid_row_end: GridLineValue::Auto,
            grid_row_start: GridLineValue::Auto,
            grid_template_areas: GridTemplateAreasValue::None,
            grid_template_columns: GridTemplateValue::None,
            grid_template_rows: GridTemplateValue::None,
        }
    }
}

pub enum GridAutoSizeValue {
    Auto,
    Custom(UnitValue), // E.g., "50px", "1fr"
}

pub enum GridAutoFlowValue {
    Row,
    Column,
    RowDense,
    ColumnDense,
}

pub enum GridLineValue {
    Auto,
    Number(i32), // Specific line
    Name(String), // Named grid line
}

pub enum GridTemplateAreasValue {
    None,
    Custom(String), // Area definitions, e.g., `"header header" "main sidebar"`
}

pub enum GridTemplateValue {
    None,
    Custom(String), // Custom definition like "repeat(3, 1fr)"
}
