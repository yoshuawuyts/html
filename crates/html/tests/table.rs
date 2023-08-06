//! Table tests for https://github.com/yoshuawuyts/html/issues/26

#![recursion_limit = "512"]

use html::tables::Table;

#[test]
fn caption() {
    Table::builder().caption(|caption| caption).build();
}

#[test]
fn colgroup() {
    Table::builder().table_column_group(|group| group).build();
}

#[test]
fn col() {
    Table::builder()
        .table_column_group(|group| group.table_column(|col| col))
        .build();
}

#[test]
fn thead() {
    Table::builder().table_head(|head| head).build();
}

#[test]
fn tbody() {
    Table::builder().table_body(|body| body).build();
}

#[test]
fn tr() {
    Table::builder().table_row(|row| row).build();
}

#[test]
fn th() {
    Table::builder()
        .table_row(|row| row.table_header(|header| header))
        .build();
}

#[test]
fn td() {
    Table::builder()
        .table_row(|row| row.table_cell(|cell| cell))
        .build();
}

#[test]
fn tfoot() {
    Table::builder().table_foot(|foot| foot).build();
}

// #[test]
// fn script_supporting() {}
