use crate::model::message::ErrorMessage;
use crate::model::products as model;

pub fn create_product(p: model::NewProduct) -> Result<model::Product, ErrorMessage> {
    let route = format!("{}/products", super::MINERVAHOST.lock().unwrap());
    let client = reqwest::blocking::Client::new();
    let response = client
        .post(route)
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
    let route = format!("{}/products", super::MINERVAHOST.lock().unwrap());
    let response = reqwest::blocking::get(route).map_err(|_| ErrorMessage::default())?;

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
