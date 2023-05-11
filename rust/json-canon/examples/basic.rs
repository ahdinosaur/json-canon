use json_canon::to_string;
use serde_json::{json, Error};

fn main() -> Result<(), Error> {
    let data = json!({
        "from_account": "543 232 625-3",
        "to_account": "321 567 636-4",
        "amount": 500,
        "currency": "USD"
    });

    println!("{}", to_string(&data)?);
    // {"amount":500,"currency":"USD","from_account":"543 232 625-3","to_account":"321 567 636-4"}

    Ok(())
}
