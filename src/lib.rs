#![allow(clippy::not_unsafe_ptr_arg_deref)]
mod msg;

use crypto_market_type::MarketType;
use crypto_msg_type::MessageType;
use msg::Message;

use std::{
    ffi::{CStr, CString},
    os::raw::{c_char, c_uint},
    sync::mpsc::Receiver,
};

// Converts an array of symbols from C to rust
fn convert_symbols(symbols: *const *const c_char, num_symbols: c_uint) -> Vec<String> {
    let mut arr = Vec::<String>::new();
    if num_symbols > 0 {
        for i in 0..num_symbols {
            let c_str = unsafe {
                let symbol_ptr: *const c_char = *(symbols.add(i as usize));
                debug_assert!(!symbol_ptr.is_null());
                CStr::from_ptr(symbol_ptr)
            };
            arr.push(c_str.to_str().unwrap().to_string());
        }
    }
    debug_assert_eq!(arr.len(), num_symbols as usize);
    arr
}

// create a thread to call on_msg to process messages.
fn process_msg(
    rx: Receiver<crypto_crawler::Message>,
    on_msg: extern "C" fn(*const Message),
) -> std::thread::JoinHandle<()> {
    std::thread::spawn(move || {
        for msg in rx {
            let exchange_cstring = CString::new(msg.exchange.clone()).unwrap();
            let json = serde_json::to_string(&msg).unwrap();
            let json_cstring = CString::new(json).unwrap();

            let msg_ffi = Message {
                exchange: exchange_cstring.as_ptr(),
                market_type: msg.market_type,
                msg_type: msg.msg_type,
                received_at: msg.received_at,
                json: json_cstring.as_ptr(),
            };

            on_msg(&msg_ffi);
        }
    })
}

macro_rules! gen_crawl_func {
    ($crawl_func_name:expr, $exchange:ident, $market_type:ident, $symbols:ident, $num_symbols:ident, $on_msg:ident, $duration:ident) => {
        let exchange_rust = unsafe {
            debug_assert!(!$exchange.is_null());
            CStr::from_ptr($exchange).to_str().unwrap()
        };

        let symbols_rust = convert_symbols($symbols, $num_symbols);

        let (tx, rx) = std::sync::mpsc::channel::<crypto_crawler::Message>();
        let process_thread = process_msg(rx, $on_msg);

        // TODO: std::panic::catch_unwind
        $crawl_func_name(
            exchange_rust,
            $market_type,
            if symbols_rust.is_empty() {
                None
            } else {
                Some(&symbols_rust)
            },
            tx,
            if $duration > 0 { Some($duration) } else { None },
        );
        process_thread.join().unwrap();
    };
}

/// Crawl realtime trades.
///
/// ## Arguments
///
/// - `exchange` The exchange name, can not be null
/// - `market_type` The market type
/// - `symbols` Symbols to crawl
/// - `num_symbols` Number of symbols, 0 means all symbols in the `market_type`
/// - `on_msg` The callback function to process messages
/// - `duration` How many seconds to run, only useful in testing, 0 means run forever
#[no_mangle]
pub extern "C" fn crawl_trade(
    exchange: *const c_char,
    market_type: MarketType,
    symbols: *const *const c_char,
    num_symbols: c_uint,
    on_msg: extern "C" fn(*const Message),
    duration: u64,
) {
    gen_crawl_func!(
        crypto_crawler::crawl_trade,
        exchange,
        market_type,
        symbols,
        num_symbols,
        on_msg,
        duration
    );
}

/// Crawl level2 orderbook update events.
#[no_mangle]
pub extern "C" fn crawl_l2_event(
    exchange: *const c_char,
    market_type: MarketType,
    symbols: *const *const c_char,
    num_symbols: c_uint,
    on_msg: extern "C" fn(*const Message),
    duration: u64,
) {
    gen_crawl_func!(
        crypto_crawler::crawl_l2_event,
        exchange,
        market_type,
        symbols,
        num_symbols,
        on_msg,
        duration
    );
}

/// Crawl level3 orderbook update events.
#[no_mangle]
pub extern "C" fn crawl_l3_event(
    exchange: *const c_char,
    market_type: MarketType,
    symbols: *const *const c_char,
    num_symbols: c_uint,
    on_msg: extern "C" fn(*const Message),
    duration: u64,
) {
    gen_crawl_func!(
        crypto_crawler::crawl_l3_event,
        exchange,
        market_type,
        symbols,
        num_symbols,
        on_msg,
        duration
    );
}

/// Crawl level2 orderbook snapshots through RESTful APIs.
#[no_mangle]
pub extern "C" fn crawl_l2_snapshot(
    exchange: *const c_char,
    market_type: MarketType,
    symbols: *const *const c_char,
    num_symbols: c_uint,
    on_msg: extern "C" fn(*const Message),
    duration: u64,
) {
    gen_crawl_func!(
        crypto_crawler::crawl_l2_snapshot,
        exchange,
        market_type,
        symbols,
        num_symbols,
        on_msg,
        duration
    );
}

/// Crawl best bid and ask.
#[no_mangle]
pub extern "C" fn crawl_bbo(
    exchange: *const c_char,
    market_type: MarketType,
    symbols: *const *const c_char,
    num_symbols: c_uint,
    on_msg: extern "C" fn(*const Message),
    duration: u64,
) {
    gen_crawl_func!(
        crypto_crawler::crawl_bbo,
        exchange,
        market_type,
        symbols,
        num_symbols,
        on_msg,
        duration
    );
}

/// Crawl level2 orderbook top-k snapshots through websocket.
#[no_mangle]
pub extern "C" fn crawl_l2_topk(
    exchange: *const c_char,
    market_type: MarketType,
    symbols: *const *const c_char,
    num_symbols: c_uint,
    on_msg: extern "C" fn(*const Message),
    duration: u64,
) {
    gen_crawl_func!(
        crypto_crawler::crawl_l2_topk,
        exchange,
        market_type,
        symbols,
        num_symbols,
        on_msg,
        duration
    );
}

/// Crawl level3 orderbook snapshots through RESTful APIs.
#[no_mangle]
pub extern "C" fn crawl_l3_snapshot(
    exchange: *const c_char,
    market_type: MarketType,
    symbols: *const *const c_char,
    num_symbols: c_uint,
    on_msg: extern "C" fn(*const Message),
    duration: u64,
) {
    gen_crawl_func!(
        crypto_crawler::crawl_l3_snapshot,
        exchange,
        market_type,
        symbols,
        num_symbols,
        on_msg,
        duration
    );
}

/// Crawl 24hr rolling window ticker.
///
/// If `symbols` is None, it means all trading symbols in the `market_type`,
/// and updates the latest symbols every hour.
#[no_mangle]
pub extern "C" fn crawl_ticker(
    exchange: *const c_char,
    market_type: MarketType,
    symbols: *const *const c_char,
    num_symbols: c_uint,
    on_msg: extern "C" fn(*const Message),
    duration: u64,
) {
    gen_crawl_func!(
        crypto_crawler::crawl_ticker,
        exchange,
        market_type,
        symbols,
        num_symbols,
        on_msg,
        duration
    );
}

/// Crawl perpetual swap funding rates.
#[no_mangle]
pub extern "C" fn crawl_funding_rate(
    exchange: *const c_char,
    market_type: MarketType,
    symbols: *const *const c_char,
    num_symbols: c_uint,
    on_msg: extern "C" fn(*const Message),
    duration: u64,
) {
    gen_crawl_func!(
        crypto_crawler::crawl_funding_rate,
        exchange,
        market_type,
        symbols,
        num_symbols,
        on_msg,
        duration
    );
}

/// Crawl candlestick(i.e., OHLCV) data.
///
/// If `symbol_interval_list` is None or empty, this API will crawl candlesticks from
/// 10 seconds to 3 minutes(if available) for all symbols.
#[no_mangle]
pub extern "C" fn crawl_candlestick(
    exchange: *const c_char,
    market_type: MarketType,
    symbols: *const *const c_char,
    intervals: *const c_uint,
    num_symbols: c_uint,
    on_msg: extern "C" fn(*const Message),
    duration: u64,
) {
    let exchange_rust = unsafe {
        debug_assert!(!exchange.is_null());
        CStr::from_ptr(exchange).to_str().unwrap()
    };

    let mut symbols_rust = Vec::<String>::new();
    let mut intervals_rust = Vec::<usize>::new();
    if num_symbols > 0 {
        for i in 0..num_symbols {
            let c_str = unsafe {
                let symbol_ptr: *const c_char = *(symbols.add(i as usize));
                debug_assert!(!symbol_ptr.is_null());
                CStr::from_ptr(symbol_ptr)
            };
            symbols_rust.push(c_str.to_str().unwrap().to_string());

            let interval: usize = unsafe { *(intervals.add(i as usize)) as usize };
            intervals_rust.push(interval);
        }
    }
    debug_assert_eq!(symbols_rust.len(), num_symbols as usize);
    debug_assert_eq!(intervals_rust.len(), num_symbols as usize);

    let symbol_interval_list: Vec<(String, usize)> = {
        symbols_rust
            .into_iter()
            .zip(intervals_rust.into_iter())
            .collect()
    };

    let (tx, rx) = std::sync::mpsc::channel::<crypto_crawler::Message>();
    let process_thread = process_msg(rx, on_msg);

    // TODO: std::panic::catch_unwind
    crypto_crawler::crawl_candlestick(
        exchange_rust,
        market_type,
        if symbol_interval_list.is_empty() {
            None
        } else {
            Some(&symbol_interval_list)
        },
        tx,
        if duration > 0 { Some(duration) } else { None },
    );
    process_thread.join().unwrap();
}

/// Crawl all open interest.
#[no_mangle]
pub extern "C" fn crawl_open_interest(
    exchange: *const c_char,
    market_type: MarketType,
    on_msg: extern "C" fn(*const Message),
    duration: u64,
) {
    let exchange_rust = unsafe {
        debug_assert!(!exchange.is_null());
        CStr::from_ptr(exchange).to_str().unwrap()
    };

    let (tx, rx) = std::sync::mpsc::channel::<crypto_crawler::Message>();
    let process_thread = process_msg(rx, on_msg);

    // TODO: std::panic::catch_unwind
    crypto_crawler::crawl_open_interest(
        exchange_rust,
        market_type,
        tx,
        if duration > 0 { Some(duration) } else { None },
    );
    process_thread.join().unwrap();
}

/// Subscribe to multiple message types of one symbol.
///
/// This API is suitable for client applications such as APP, website, etc.
///
/// String messages in `tx` are already parsed by `crypto-msg-parser`.
#[no_mangle]
pub extern "C" fn subscribe_symbol(
    exchange: *const c_char,
    market_type: MarketType,
    symbol: *const c_char,
    msg_types: *const MessageType,
    num_msg_types: c_uint,
    on_msg: extern "C" fn(*const c_char),
    duration: u64,
) {
    let exchange_rust = unsafe {
        debug_assert!(!exchange.is_null());
        CStr::from_ptr(exchange).to_str().unwrap()
    };
    let symbol_rust = unsafe {
        debug_assert!(!symbol.is_null());
        CStr::from_ptr(symbol).to_str().unwrap()
    };
    let msg_type_array = {
        let mut arr = Vec::<MessageType>::new();
        if num_msg_types > 0 {
            for i in 0..num_msg_types {
                let msg_type: MessageType = unsafe { *(msg_types.add(i as usize)) };
                arr.push(msg_type);
            }
        }
        debug_assert_eq!(arr.len(), num_msg_types as usize);
        arr
    };

    let (tx, rx) = std::sync::mpsc::channel::<String>();
    let process_thread = std::thread::spawn(move || {
        for json in rx {
            let json_cstring = CString::new(json).unwrap();
            on_msg(json_cstring.as_ptr());
        }
    });

    // TODO: std::panic::catch_unwind
    crypto_crawler::subscribe_symbol(
        exchange_rust,
        market_type,
        symbol_rust,
        &msg_type_array,
        tx,
        if duration > 0 { Some(duration) } else { None },
    );
    process_thread.join().unwrap();
}
