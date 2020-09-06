use serde_json::{Result, Value};

pub fn validate_json() {
    log::info!("Ok!")
}

pub fn parse_to_result(data: &String) -> Result<()> {
    let v: Value = serde_json::from_str(data)?;
    Ok(())
}