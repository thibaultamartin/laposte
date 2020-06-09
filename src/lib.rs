use crate::errors::ClientError::*;
use crate::errors::ClientResult;
use crate::status::{DeliveryStatus, EventStatus};

use regex::Regex;
use serde::{Serialize, Deserialize, de};
use std::convert::TryFrom;
use surf::http;

pub mod errors;
pub mod status;

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TimelineEvent {
    pub id: u32,
    pub short_label: String,
    pub long_label: String,
    pub status: bool,
    pub r#type: u32,
    pub country: String
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Event {
    pub order: u32,
    pub date: String,
    pub label: String,
    #[serde(deserialize_with = "event_status")]
    pub code: EventStatus,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Shipment {
    pub id_ship: String,
    pub product: String,
    pub is_final: bool,
    #[serde(deserialize_with = "delivery_status")]
    pub timeline: DeliveryStatus,
    pub event: Vec<Event>,
    pub holder: u32,
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
pub struct TrackingNumber {
    tracking_number: String,
}

impl TryFrom<&str> for TrackingNumber {
    type Error = &'static str;

    fn try_from(tracking_number: &str) -> Result<Self, Self::Error> {
        // Tacking number can be either
        // 1 number + 1 letter + 11 numbers     or
        // 2 letters + 11 numbers               or
        // 2 letters + 9 numbers + 2 letters    or
        // 15 numbers                           or
        // 14 numbers + 1 letter
        let re = Regex::new(r"^(\d[a-zA-Z]\d{11})$|^([a-zA-Z]{2}\d{11})$|^([a-zA-Z]{2}\d{9}[a-zA-Z]{2})$|^(\d{15})$|^(\d{14}[a-zA-Z])$").unwrap();

        if !re.is_match(tracking_number) { return Err("Tracking number did not match La Poste format") }
        else { Ok(TrackingNumber{ tracking_number: tracking_number.trim().to_string() }) }
    }
}

impl Into<String> for TrackingNumber {
    fn into(self) -> String {
        self.tracking_number
    }
}

impl TrackingNumber {
    pub fn to_string(&self) -> String {
        self.tracking_number.clone()
    }
}

impl Shipment {
    pub fn last_event(&self) -> Option<&Event> {
        self.event.iter().max_by(|a, b| {
            a.order.cmp(&b.order)
        })
    }

    pub fn first_event(&self) -> Option<&Event> {
        self.event.iter().min_by(|a, b| {
            a.order.cmp(&b.order)
        })
    }

    pub fn shipping_event(&self) -> Option<&Event> {
        // No event -> it's not worth checking
        if self.event.len() == 0 {
            return None
        }

        // Try to return the first EventStatus::{Declared,  CollectedByCarrier,
        // CollectedInShippingCountry} we can find
        let declared: Vec<&Event> = self.event.iter().filter(|event| {event.code == EventStatus::Declared}).collect();
        if declared.len() > 0 {
            // TODO if there are several entries, sort by date
            return Some(declared.iter().next().unwrap())
        }

        let collected_by_carrier: Vec<&Event> = self.event.iter().filter(|event| {event.code == EventStatus::CollectedByCarrier}).collect();
        if collected_by_carrier.len() > 0 {
            // TODO if there are several entries, sort by date
            return Some(collected_by_carrier.iter().next().unwrap())
        }

        let collected_in_shipping_country: Vec<&Event> = self.event.iter().filter(|event| {event.code == EventStatus::CollectedInShippingCountry}).collect();
        if collected_in_shipping_country.len() > 0 {
            return Some(collected_in_shipping_country.iter().next().unwrap())
        }

        None
    }
}

#[derive(Clone)]
pub struct Client {
    okapi_key: String,
}

impl Client {
    pub fn new(okapi_key: &str) -> Client {
        Client { okapi_key: okapi_key.to_string() }
    }
    pub async fn get_tracking_info(&self, tracking_number: TrackingNumber) -> ClientResult<TrackingInfo> {
        let mut uri = "https://api.laposte.fr/suivi/v2/idships/".to_string();
        uri.push_str(&tracking_number.to_string());
        let mut response = surf::get(uri)
            .set_header("Accept", "application/json")
            .set_header("X-Okapi-Key", &(self.okapi_key))
            .await?;

        match response.status() {
            http::StatusCode::OK => {},
            http::StatusCode::BAD_REQUEST => return Err(InvalidFormat),
            http::StatusCode::UNAUTHORIZED => return Err(Unauthorized),
            http::StatusCode::NOT_FOUND => return Err(ParcelNotFound),
            _ => return Err(ServerError),
        };
    
        let res: TrackingInfo = response.body_json().await?;
        Ok(res)
    }
}

fn delivery_status<'de, D>(deserializer: D) -> Result<DeliveryStatus, D::Error>
where
    D: de::Deserializer<'de>
{
    let timeline = Vec::<TimelineEvent>::deserialize(deserializer)?;
    let mut step = 1;

    for event in timeline {
        if event.status {
            step = event.id;
        } else {
            break;
        }
    }

    DeliveryStatus::try_from(step).map_err(de::Error::custom)
}

fn event_status<'de, D>(deserializer:D) -> Result<EventStatus, D::Error>
where
    D: de::Deserializer<'de>
{
    let code = String::deserialize(deserializer)?;

    EventStatus::try_from(code.as_ref()).map_err(de::Error::custom)
}
