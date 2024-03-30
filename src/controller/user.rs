use crate::model::message::ErrorMessage;
use crate::model::user as model;

pub fn get_user_index() -> Result<Vec<model::User>, ErrorMessage> {
    let route = format!("{}/users", super::MINERVAHOST.lock().unwrap());
    let response = reqwest::blocking::get(route).map_err(|_| ErrorMessage::default())?;

    match response.status() {
        reqwest::StatusCode::OK => match response.json::<Vec<model::User>>() {
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

pub fn create_user(u: model::NewUser) -> Result<model::User, ErrorMessage> {
    let route = format!("{}/users", super::MINERVAHOST.lock().unwrap());
    let client = reqwest::blocking::Client::new();
    let response = client
        .post(route)
        .json(&u)
        .send()
        .map_err(|_| ErrorMessage::default())?;

    match response.status() {
        reqwest::StatusCode::CREATED => match response.json::<model::User>() {
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

pub fn remove_user(id: uuid::Uuid) -> Result<(), ErrorMessage> {
    let route = format!("{}/users/{}", super::MINERVAHOST.lock().unwrap(), id);
    let client = reqwest::blocking::Client::new();
    let response = client
        .delete(route)
        .send()
        .map_err(|_| ErrorMessage::default())?;

    match response.status() {
        reqwest::StatusCode::OK => Ok(()),
        _ => match response.json::<ErrorMessage>() {
            Ok(e) => Err(e),
            Err(e) => Err(ErrorMessage::internal(
                "Could not parse error message",
                Some(e.to_string()),
            )),
        },
    }
}
