/// Market type.
#[repr(C)]
#[derive(Copy, Clone)]
pub enum MarketType {
    Spot,
    LinearFuture,
    InverseFuture,
    LinearSwap,
    InverseSwap,

    AmericanOption,
    EuropeanOption,

    QuantoFuture,
    QuantoSwap,

    Move,
    #[allow(clippy::upper_case_acronyms)]
    BVOL,
}

impl MarketType {
    // Converts C MarketType to Rust MarketType
    pub fn to_rust(self) -> crypto_crawler::MarketType {
        match self {
            MarketType::Spot => crypto_crawler::MarketType::Spot,
            MarketType::LinearFuture => crypto_crawler::MarketType::LinearFuture,
            MarketType::InverseFuture => crypto_crawler::MarketType::InverseFuture,
            MarketType::LinearSwap => crypto_crawler::MarketType::LinearSwap,
            MarketType::InverseSwap => crypto_crawler::MarketType::InverseSwap,
            MarketType::AmericanOption => crypto_crawler::MarketType::AmericanOption,
            MarketType::EuropeanOption => crypto_crawler::MarketType::EuropeanOption,
            MarketType::QuantoFuture => crypto_crawler::MarketType::QuantoFuture,
            MarketType::QuantoSwap => crypto_crawler::MarketType::QuantoSwap,
            MarketType::Move => crypto_crawler::MarketType::Move,
            MarketType::BVOL => crypto_crawler::MarketType::BVOL,
        }
    }

    // Converts Rust MarketType to C MarketType
    pub fn from_rust(market_type: crypto_crawler::MarketType) -> Self {
        match market_type {
            crypto_crawler::MarketType::Spot => MarketType::Spot,
            crypto_crawler::MarketType::LinearFuture => MarketType::LinearFuture,
            crypto_crawler::MarketType::InverseFuture => MarketType::InverseFuture,
            crypto_crawler::MarketType::LinearSwap => MarketType::LinearSwap,
            crypto_crawler::MarketType::InverseSwap => MarketType::InverseSwap,
            crypto_crawler::MarketType::AmericanOption => MarketType::AmericanOption,
            crypto_crawler::MarketType::EuropeanOption => MarketType::EuropeanOption,
            crypto_crawler::MarketType::QuantoFuture => MarketType::QuantoFuture,
            crypto_crawler::MarketType::QuantoSwap => MarketType::QuantoSwap,
            crypto_crawler::MarketType::Move => MarketType::Move,
            crypto_crawler::MarketType::BVOL => MarketType::BVOL,
        }
    }
}
