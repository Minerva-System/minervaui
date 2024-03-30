use crate::model::message::ErrorMessage;
use crate::model::products as model;

pub fn create_product(p: model::NewProduct) -> Result<model::Product, ErrorMessage> {
    let client = reqwest::blocking::Client::new();
    let response = client
        .post("http://192.168.3.6:30000/api/v1/products")
        .json(&p)
        .send()
        .map_err(|_| ErrorMessage::default())?;

    match response.status() {
        reqwest::StatusCode::CREATED => match response.json::<model::Product>() {
            Ok(parsed) => Ok(parsed),
            Err(e) => Err(ErrorMessage::internal(
                "Could not parse data",
                Some(e.to_string()),
            )),
        },
        _ => match response.json::<ErrorMessage>() {
            Ok(e) => Err(e),
            Err(e) => Err(ErrorMessage::internal(
                "Could not parse error message",
                Some(e.to_string()),
            )),
        },
    }
}

pub fn get_product_index() -> Result<Vec<model::Product>, ErrorMessage> {
    // Run request
    let response = reqwest::blocking::get("http://192.168.3.6:30000/api/v1/products")
        .map_err(|_| ErrorMessage::default())?;

    match response.status() {
        reqwest::StatusCode::OK => match response.json::<Vec<model::Product>>() {
            Ok(parsed) => Ok(parsed),
            Err(e) => Err(ErrorMessage::internal(
                "Could not parse data",
                Some(e.to_string()),
            )),
        },
        _ => match response.json::<ErrorMessage>() {
            Ok(e) => Err(e),
            Err(e) => Err(ErrorMessage::internal(
                "Could not parse error message",
                Some(e.to_string()),
            )),
        },
    }
}
