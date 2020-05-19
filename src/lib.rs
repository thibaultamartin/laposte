use serde::{Serialize, Deserialize};

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

pub struct Client {
    okapi_key: String,
}

impl Client {
    pub fn new(okapi_key: String) -> Client {
        Client { okapi_key }
    }
    pub async fn get_tracking_info(&self, tracking_number: String) -> Result<TrackingInfo,surf::Exception> {
        let req = surf::get(format!("{}{}","https://api.laposte.fr/suivi/v2/idships/",tracking_number))
            .set_header("Accept", "application/json")
            .set_header("X-Okapi-Key", &(self.okapi_key));
    
        let res: TrackingInfo = req.recv_json().await?;
        Ok(res)
    }
}



