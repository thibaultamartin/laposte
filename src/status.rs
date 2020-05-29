use std::convert::TryFrom;
use serde::{Serialize,Deserialize};

#[derive(Debug,Serialize,Deserialize)]
pub enum DeliveryStatus {
    ReadyToBeCollectedByCarrier,
    CollectedByCarrier,
    InTransit,
    ReadyForDelivery,
    Delivered,
}

impl TryFrom<u32> for DeliveryStatus {
    type Error = &'static str;

    fn try_from(step: u32) -> Result<Self, Self::Error> {
        match step {
            1 => Ok(DeliveryStatus::ReadyToBeCollectedByCarrier),
            2 => Ok(DeliveryStatus::CollectedByCarrier),
            3 => Ok(DeliveryStatus::InTransit),
            4 => Ok(DeliveryStatus::ReadyForDelivery),
            5 => Ok(DeliveryStatus::Delivered),
            _ => Err("Not a valid step"),
        }
    }
}