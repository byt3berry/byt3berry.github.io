use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::{Document, HtmlElement, HtmlTableCellElement, HtmlTableElement, HtmlTableRowElement, HtmlTableSectionElement, Window};

struct Person {
    name: String,
    age: usize,
    city: String,
}

// Called when the Wasm module is instantiated
#[wasm_bindgen(start)]
fn main() -> Result<(), JsValue> {
    let persons = [
        Person { name: String::from("John Doe"), age: 30, city: String::from("New York") },
        Person { name: String::from("Jane Smith"), age: 25, city: String::from("Los Angeles") },
        Person { name: String::from("Mike Johnson"), age: 35, city: String::from("Chicago") }
    ];

    let window: Window = web_sys::window().expect("no global `window` exists");
    let document: Document = window.document().expect("should have a document on window");
    let body: HtmlElement = document.body().expect("document should have a body");

    // let val = document.create_element("p")?;
    // val.set_inner_html("Hello from Rust!");

    // body.append_child(&val)?;
    
    let table: HtmlTableElement = document
        .create_element("table")?
        .dyn_into()?;
    table.set_attribute("border", "1")?;

    let header: HtmlTableSectionElement = table
        .create_t_head()
        .dyn_into()?;
    let header_row: HtmlTableRowElement = header
        .insert_row()?
        .dyn_into()?;
    let headers: [&str; 3] = ["Name", "Age", "City"];
    for header_text in headers {
        let th: HtmlTableCellElement = document
            .create_element("th")?
            .dyn_into()?;
            th.set_text_content(Some(header_text));
            header_row.append_child(&th)?;
    };

    let tbody: HtmlTableSectionElement = table
        .create_t_body()
        .dyn_into()?;

    for person in persons {
        let row: HtmlTableRowElement = tbody.insert_row()?.dyn_into()?;
        insert_row(&row, &person.name)?;
        insert_row(&row, &person.age.to_string())?;
        insert_row(&row, &person.city)?;
    }

    if let Some(node) = document.get_element_by_id("table-container") {
        node.append_child(&table)?;
    }

    Ok(())
}

fn insert_row(row: &HtmlTableRowElement, value: &str) -> Result<(), JsValue> {
    let cell: HtmlTableCellElement = row
        .insert_cell()?
        .dyn_into()?;
    cell.set_text_content(Some(value));

    Ok(())
}
