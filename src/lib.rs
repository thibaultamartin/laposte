use serde::{Serialize, Deserialize};
use surf::http;
use crate::errors::ClientError::*;
use crate::errors::ClientResult;

pub mod errors;

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TimelineEvent {
    pub id: u32,
    pub short_label: String,
    pub long_label: String,
    pub status: bool,
    pub r#type: u32,
    pub country: String
}

#[derive(Deserialize, Serialize)]
pub struct Event {
    pub order: u32,
    pub date: String,
    pub label: String,
    pub code: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Shipment {
    pub id_ship: String,
    pub product: String,
    pub is_final: bool,
    pub timeline: Vec<TimelineEvent>,
    pub event: Vec<Event>,
    pub url: String,
    pub holder: u32,
    pub url_detail: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackingInfo {
    pub lang: String,
    pub scope: String,
    pub return_code: u32,
    pub shipment: Shipment,
}

#[derive(Clone)]
pub struct Client {
    okapi_key: String,
}

impl Client {
    pub fn new(okapi_key: &str) -> Client {
        Client { okapi_key: okapi_key.to_string() }
    }
    pub async fn get_tracking_info(&self, tracking_number: &str) -> ClientResult<TrackingInfo> {
        let mut response = surf::get(format!("{}{}","https://api.laposte.fr/suivi/v2/idships/",tracking_number))
            .set_header("Accept", "application/json")
            .set_header("X-Okapi-Key", &(self.okapi_key))
            .await?;

        match response.status() {
            http::StatusCode::OK => println!("Ok"),
            http::StatusCode::BAD_REQUEST => return Err(InvalidFormat),
            http::StatusCode::UNAUTHORIZED => return Err(Unauthorized),
            http::StatusCode::NOT_FOUND => return Err(ParcelNotFound),
            _ => return Err(ServerError),
        };

        println!("Response status: {}", response.status());
    
        let res: TrackingInfo = response.body_json().await?;
        Ok(res)
    }
}



