use crate::market_type::MarketType;
use std::os::raw::c_char;

/// The type of a message
#[repr(C)]
pub enum MessageType {
    Trade,
    L2Event,
    L2Snapshot,
    L3Event,
    L3Snapshot,
    #[allow(clippy::upper_case_acronyms)]
    BBO,
    Ticker,
    Candlestick,
    FundingRate,
}

impl MessageType {
    // Converts Rust MessageType to C MessageType
    pub fn from_rust(msg_type: crypto_crawler::MessageType) -> Self {
        match msg_type {
            crypto_crawler::MessageType::Trade => MessageType::Trade,
            crypto_crawler::MessageType::L2Event => MessageType::L2Event,
            crypto_crawler::MessageType::L2Snapshot => MessageType::L2Snapshot,
            crypto_crawler::MessageType::L3Event => MessageType::L3Event,
            crypto_crawler::MessageType::L3Snapshot => MessageType::L3Snapshot,
            crypto_crawler::MessageType::BBO => MessageType::BBO,
            crypto_crawler::MessageType::Ticker => MessageType::Ticker,
            crypto_crawler::MessageType::Candlestick => MessageType::Candlestick,
            crypto_crawler::MessageType::FundingRate => MessageType::FundingRate,
        }
    }
}

/// Message represents messages received by crawlers.
#[repr(C)]
pub struct Message {
    /// The exchange name, unique for each exchage
    pub exchange: *const c_char,
    /// Market type
    pub market_type: MarketType,
    /// Message type
    pub msg_type: MessageType,
    /// Unix timestamp in milliseconds
    pub received_at: u64,
    /// the original message
    pub json: *const c_char,
}
