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

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub enum EventStatus {
    Declared,
    CollectedByCarrier,
    CollectedInShippingCountry,
    UnderTreatment,
    UnderTreatmentInShippingCountry,
    UnderTreatmentInDestinationCountry,
    UnderTreatmentInTransitCountry,
    WaitingForPresentation,
    HandedToCustoms,
    ReleasedByCustoms,
    HeldByCustoms,
    ProblemOccurring,
    ProblemSolved,
    SetForDistribution,
    CannotDistribute,
    WaitingInPostOffice,
    ReturnedToSender,
    Delivered,
    DeliveredToSender,
}

impl TryFrom<&str> for EventStatus {
    type Error = &'static str;

    fn try_from(status: &str) -> Result<Self, Self::Error> {
        match status {
            "DR1" => Ok(EventStatus::Declared),
            "PC1" => Ok(EventStatus::CollectedByCarrier),
            "PC2" => Ok(EventStatus::CollectedInShippingCountry),
            "ET1" => Ok(EventStatus::UnderTreatment),
            "ET2" => Ok(EventStatus::UnderTreatmentInShippingCountry),
            "ET3" => Ok(EventStatus::UnderTreatmentInDestinationCountry),
            "ET4" => Ok(EventStatus::UnderTreatmentInTransitCountry),
            "EP1" => Ok(EventStatus::WaitingForPresentation),
            "DO1" => Ok(EventStatus::HandedToCustoms),
            "DO2" => Ok(EventStatus::ReleasedByCustoms),
            "DO3" => Ok(EventStatus::HeldByCustoms),
            "PB1" => Ok(EventStatus::ProblemOccurring),
            "PB2" => Ok(EventStatus::ProblemSolved),
            "MD2" => Ok(EventStatus::SetForDistribution),
            "ND1" => Ok(EventStatus::CannotDistribute),
            "AG1" => Ok(EventStatus::WaitingInPostOffice),
            "RE1" => Ok(EventStatus::ReturnedToSender),
            "DI1" => Ok(EventStatus::Delivered),
            "DI2" => Ok(EventStatus::DeliveredToSender),
            _ => Err("Not a valid timeline event code"),
        }
    }
}
