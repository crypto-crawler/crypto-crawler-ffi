use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;
use std::os::raw::c_char;

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
