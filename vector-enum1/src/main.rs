enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    let val1 = row.get(0);
    match val1 {
        None => {},
        Some(x) => {
            match x {
                SpreadsheetCell::Int(_) => {}
                SpreadsheetCell::Float(_) => {}
                SpreadsheetCell::Text(_) => {}
            }
        }
    }
}
