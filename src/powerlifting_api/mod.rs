use api_reader::get_powerlifters;
use powerlifter::Powerlifter;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::{Document, Element, HtmlTableCellElement, HtmlTableElement, HtmlTableRowElement, HtmlTableSectionElement, HtmlTextAreaElement, Window};

mod api_reader;
mod powerlifter;

const HEADERS: [&str; 12] = ["Rank", "Lifter", "Federation", "Sex", "Age", "Equipment", "Class", "Weight", "Squat", "Bench", "Deadlift", "Total"];

#[wasm_bindgen]
pub async fn run() -> Result<(), JsValue> {
    let window: Window = web_sys::window().expect("No global \"window\" exists");
    let document: Document = window.document().expect("The window should have a document");
    let input: Vec<String> = read_input(&document)?;
    let powerlifters = get_powerlifters(input);

    let table: HtmlTableElement = document
        .create_element("table")?
        .dyn_into()?;
    table.set_attribute("border", "1")?;

    setup_table_headers(&document, &table)?;

    let table_body: HtmlTableSectionElement = table
        .create_t_body()
        .dyn_into()?;

    for powerlifter in powerlifters.await {
        insert_row(&table_body, powerlifter)?;
    }

    if let Some(node) = document.get_element_by_id("table-container") {
        empty_element(&node)?;
        node.append_child(&table)?;
    }

    Ok(())
}

fn read_input(document: &Document) -> Result<Vec<String>, JsValue> {
    let Some(input) = document.get_element_by_id("input") else {
        return Err(JsValue::from("No element with id \"input\""));
    };

    let input: HtmlTextAreaElement = input.dyn_into()?;
    let output: Vec<String> = input
        .value()
        .lines()
        .map(|line| line.to_string())
        .collect();
    Ok(output)
}

fn setup_table_headers(document: &Document, table: &HtmlTableElement) -> Result<(), JsValue> {
    let header: HtmlTableSectionElement = table
        .create_t_head()
        .dyn_into()?;
    let header_row: HtmlTableRowElement = header
        .insert_row()?
        .dyn_into()?;
    for header_text in HEADERS {
        let th: HtmlTableCellElement = document
            .create_element("th")?
            .dyn_into()?;
            th.set_text_content(Some(header_text));
            header_row.append_child(&th)?;
    };

    Ok(())
}

fn empty_element(element: &Element) -> Result<(), JsValue> {
    let mut child;

    while element.has_child_nodes() {
        child = element.first_child().expect("The element should have at least one element");
        element.remove_child(&child)?;
    }

    Ok(())
}

fn insert_row(table_body: &HtmlTableSectionElement, powerlifter: (String, Option<Powerlifter>)) -> Result<(), JsValue> {
    let row: HtmlTableRowElement = table_body.insert_row()?.dyn_into()?;
    let (name, powerlifter) = powerlifter;

    if let Some(powerlifter) = powerlifter {
        insert_column(&row, &powerlifter.rank.to_string())?;
        insert_column(&row, &powerlifter.name)?;
        insert_column(&row, &powerlifter.federation)?;
        insert_column(&row, &powerlifter.sex)?;
        insert_column(&row, &powerlifter.age)?;
        insert_column(&row, &powerlifter.equipment)?;
        insert_column(&row, &powerlifter.weightclass)?;
        insert_column(&row, &powerlifter.bodyweight)?;
        insert_column(&row, &powerlifter.squat)?;
        insert_column(&row, &powerlifter.bench)?;
        insert_column(&row, &powerlifter.deadlift)?;
        insert_column(&row, &powerlifter.total)?;
    } else {
        insert_column(&row, "?")?;
        insert_column(&row, &name)?;
        insert_column(&row, "?")?;
        insert_column(&row, "?")?;
        insert_column(&row, "?")?;
        insert_column(&row, "?")?;
        insert_column(&row, "?")?;
        insert_column(&row, "?")?;
        insert_column(&row, "?")?;
        insert_column(&row, "?")?;
        insert_column(&row, "?")?;
        insert_column(&row, "?")?;
    }

    Ok(())
}

fn insert_column(row: &HtmlTableRowElement, value: &str) -> Result<(), JsValue> {
    let cell: HtmlTableCellElement = row
        .insert_cell()?
        .dyn_into()?;
    cell.set_text_content(Some(value));

    Ok(())
}
