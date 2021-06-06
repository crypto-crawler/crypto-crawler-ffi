#![allow(clippy::not_unsafe_ptr_arg_deref)]
mod msg;
pub use msg::*;

mod market_type;
pub use market_type::*;

use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
    sync::{Arc, Mutex},
};

// Converts an array of symbols from C to rust
fn convert_symbols(symbols: *const *const c_char, num_symbols: usize) -> Vec<String> {
    let mut arr = Vec::<String>::new();
    if num_symbols > 0 {
        for i in 0..num_symbols {
            let c_str = unsafe {
                let symbol_ptr: *const c_char = *(symbols.add(i));
                debug_assert!(!symbol_ptr.is_null());
                CStr::from_ptr(symbol_ptr)
            };
            arr.push(c_str.to_str().unwrap().to_string());
        }
    }
    debug_assert_eq!(arr.len(), num_symbols);
    arr
}

fn process_msg(on_msg: extern "C" fn(*const Message), msg: crypto_crawler::Message) {
    let exchange_cstring = CString::new(msg.exchange).unwrap();
    let json_cstring = CString::new(msg.json).unwrap();

    let msg_ffi = Message {
        exchange: exchange_cstring.as_ptr(),
        market_type: MarketType::from_rust(msg.market_type),
        msg_type: MessageType::from_rust(msg.msg_type),
        received_at: msg.received_at,
        json: json_cstring.as_ptr(),
    };
    on_msg(&msg_ffi);
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
/// - `duration` How many seconds to run, only useful in testing
#[no_mangle]
pub extern "C" fn crawl_trade(
    exchange: *const c_char,
    market_type: MarketType,
    symbols: *const *const c_char,
    num_symbols: usize,
    on_msg: extern "C" fn(*const Message),
    duration: u64,
) {
    let c_str = unsafe {
        debug_assert!(!exchange.is_null());
        CStr::from_ptr(exchange)
    };
    let exchange_rust = c_str.to_str().unwrap();

    let symbols_rust = convert_symbols(symbols, num_symbols);

    let on_msg_ext = Arc::new(Mutex::new(move |msg: crypto_crawler::Message| {
        process_msg(on_msg, msg);
    }));

    let result = std::panic::catch_unwind(|| {
        crypto_crawler::crawl_trade(
            exchange_rust,
            market_type.to_rust(),
            if symbols_rust.is_empty() {
                None
            } else {
                Some(&symbols_rust)
            },
            on_msg_ext,
            if duration > 0 { Some(duration) } else { None },
        );
    });
    if let Err(err) = result {
        eprintln!("{:?}", err);
    }
}

/// Crawl level2 orderbook update events.
#[no_mangle]
pub extern "C" fn crawl_l2_event(
    exchange: *const c_char,
    market_type: MarketType,
    symbols: *const *const c_char,
    num_symbols: usize,
    on_msg: extern "C" fn(*const Message),
    duration: u64,
) {
    let c_str = unsafe {
        debug_assert!(!exchange.is_null());
        CStr::from_ptr(exchange)
    };
    let exchange_rust = c_str.to_str().unwrap();

    let symbols_rust = convert_symbols(symbols, num_symbols);

    let on_msg_ext = Arc::new(Mutex::new(move |msg: crypto_crawler::Message| {
        process_msg(on_msg, msg);
    }));

    let result = std::panic::catch_unwind(|| {
        crypto_crawler::crawl_l2_event(
            exchange_rust,
            market_type.to_rust(),
            if symbols_rust.is_empty() {
                None
            } else {
                Some(&symbols_rust)
            },
            on_msg_ext,
            if duration > 0 { Some(duration) } else { None },
        );
    });
    if let Err(err) = result {
        eprintln!("{:?}", err);
    }
}

/// Crawl level2 orderbook snapshots through RESTful APIs.
#[no_mangle]
pub extern "C" fn crawl_l2_snapshot(
    exchange: *const c_char,
    market_type: MarketType,
    symbols: *const *const c_char,
    num_symbols: usize,
    on_msg: extern "C" fn(*const Message),
    interval: u64,
    duration: u64,
) {
    let c_str = unsafe {
        debug_assert!(!exchange.is_null());
        CStr::from_ptr(exchange)
    };
    let exchange_rust = c_str.to_str().unwrap();

    let symbols_rust = convert_symbols(symbols, num_symbols);

    let on_msg_ext = Arc::new(Mutex::new(move |msg: crypto_crawler::Message| {
        process_msg(on_msg, msg);
    }));

    let result = std::panic::catch_unwind(|| {
        crypto_crawler::crawl_l2_snapshot(
            exchange_rust,
            market_type.to_rust(),
            if symbols_rust.is_empty() {
                None
            } else {
                Some(&symbols_rust)
            },
            on_msg_ext,
            Some(interval),
            if duration > 0 { Some(duration) } else { None },
        );
    });
    if let Err(err) = result {
        eprintln!("{:?}", err);
    }
}

/// Crawl level3 orderbook update events.
#[no_mangle]
pub extern "C" fn crawl_l3_event(
    exchange: *const c_char,
    market_type: MarketType,
    symbols: *const *const c_char,
    num_symbols: usize,
    on_msg: extern "C" fn(*const Message),
    duration: u64,
) {
    let c_str = unsafe {
        debug_assert!(!exchange.is_null());
        CStr::from_ptr(exchange)
    };
    let exchange_rust = c_str.to_str().unwrap();

    let symbols_rust = convert_symbols(symbols, num_symbols);

    let on_msg_ext = Arc::new(Mutex::new(move |msg: crypto_crawler::Message| {
        process_msg(on_msg, msg);
    }));

    let result = std::panic::catch_unwind(|| {
        crypto_crawler::crawl_l3_event(
            exchange_rust,
            market_type.to_rust(),
            if symbols_rust.is_empty() {
                None
            } else {
                Some(&symbols_rust)
            },
            on_msg_ext,
            if duration > 0 { Some(duration) } else { None },
        );
    });
    if let Err(err) = result {
        eprintln!("{:?}", err);
    }
}

/// Crawl level3 orderbook snapshots through RESTful APIs.
#[no_mangle]
pub extern "C" fn crawl_l3_snapshot(
    exchange: *const c_char,
    market_type: MarketType,
    symbols: *const *const c_char,
    num_symbols: usize,
    on_msg: extern "C" fn(*const Message),
    interval: u64,
    duration: u64,
) {
    let c_str = unsafe {
        debug_assert!(!exchange.is_null());
        CStr::from_ptr(exchange)
    };
    let exchange_rust = c_str.to_str().unwrap();

    let symbols_rust = convert_symbols(symbols, num_symbols);

    let on_msg_ext = Arc::new(Mutex::new(move |msg: crypto_crawler::Message| {
        process_msg(on_msg, msg);
    }));

    let result = std::panic::catch_unwind(|| {
        crypto_crawler::crawl_l3_snapshot(
            exchange_rust,
            market_type.to_rust(),
            if symbols_rust.is_empty() {
                None
            } else {
                Some(&symbols_rust)
            },
            on_msg_ext,
            Some(interval),
            if duration > 0 { Some(duration) } else { None },
        );
    });
    if let Err(err) = result {
        eprintln!("{:?}", err);
    }
}
