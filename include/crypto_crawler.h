/* Licensed under Apache-2.0 */

#ifndef CRYPTO_CRAWLER_H_
#define CRYPTO_CRAWLER_H_

/* Generated with cbindgen:0.20.0 */

/* Warning, this file is autogenerated by cbindgen. Don't modify this manually. */

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include "crypto_market_type.h"
#include "crypto_msg_type.h"

/**
 * Message represents messages received by crawlers.
 */
typedef struct {
  /**
   * The exchange name, unique for each exchage
   */
  const char *exchange;
  /**
   * Market type
   */
  MarketType market_type;
  /**
   * Message type
   */
  MessageType msg_type;
  /**
   * Unix timestamp in milliseconds
   */
  uint64_t received_at;
  /**
   * the original message
   */
  const char *json;
} Message;

#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

/**
 * Crawl realtime trades.
 *
 * ## Arguments
 *
 * - `exchange` The exchange name, can not be null
 * - `market_type` The market type
 * - `symbols` Symbols to crawl
 * - `num_symbols` Number of symbols, 0 means all symbols in the `market_type`
 * - `on_msg` The callback function to process messages
 * - `duration` How many seconds to run, only useful in testing, 0 means run forever
 */
void crawl_trade(const char *exchange,
                 MarketType market_type,
                 const char *const *symbols,
                 unsigned int num_symbols,
                 void (*on_msg)(const Message*),
                 uint64_t duration);

/**
 * Crawl level2 orderbook update events.
 */
void crawl_l2_event(const char *exchange,
                    MarketType market_type,
                    const char *const *symbols,
                    unsigned int num_symbols,
                    void (*on_msg)(const Message*),
                    uint64_t duration);

/**
 * Crawl level3 orderbook update events.
 */
void crawl_l3_event(const char *exchange,
                    MarketType market_type,
                    const char *const *symbols,
                    unsigned int num_symbols,
                    void (*on_msg)(const Message*),
                    uint64_t duration);

/**
 * Crawl level2 orderbook snapshots through RESTful APIs.
 */
void crawl_l2_snapshot(const char *exchange,
                       MarketType market_type,
                       const char *const *symbols,
                       unsigned int num_symbols,
                       void (*on_msg)(const Message*),
                       uint64_t duration);

/**
 * Crawl best bid and ask.
 */
void crawl_bbo(const char *exchange,
               MarketType market_type,
               const char *const *symbols,
               unsigned int num_symbols,
               void (*on_msg)(const Message*),
               uint64_t duration);

/**
 * Crawl level2 orderbook top-k snapshots through websocket.
 */
void crawl_l2_topk(const char *exchange,
                   MarketType market_type,
                   const char *const *symbols,
                   unsigned int num_symbols,
                   void (*on_msg)(const Message*),
                   uint64_t duration);

/**
 * Crawl level3 orderbook snapshots through RESTful APIs.
 */
void crawl_l3_snapshot(const char *exchange,
                       MarketType market_type,
                       const char *const *symbols,
                       unsigned int num_symbols,
                       void (*on_msg)(const Message*),
                       uint64_t duration);

/**
 * Crawl 24hr rolling window ticker.
 *
 * If `symbols` is None, it means all trading symbols in the `market_type`,
 * and updates the latest symbols every hour.
 */
void crawl_ticker(const char *exchange,
                  MarketType market_type,
                  const char *const *symbols,
                  unsigned int num_symbols,
                  void (*on_msg)(const Message*),
                  uint64_t duration);

/**
 * Crawl perpetual swap funding rates.
 */
void crawl_funding_rate(const char *exchange,
                        MarketType market_type,
                        const char *const *symbols,
                        unsigned int num_symbols,
                        void (*on_msg)(const Message*),
                        uint64_t duration);

/**
 * Crawl candlestick(i.e., OHLCV) data.
 *
 * If `symbol_interval_list` is None or empty, this API will crawl candlesticks from
 * 10 seconds to 3 minutes(if available) for all symbols.
 */
void crawl_candlestick(const char *exchange,
                       MarketType market_type,
                       const char *const *symbols,
                       const unsigned int *intervals,
                       unsigned int num_symbols,
                       void (*on_msg)(const Message*),
                       uint64_t duration);

/**
 * Crawl all open interest.
 */
void crawl_open_interest(const char *exchange,
                         MarketType market_type,
                         void (*on_msg)(const Message*),
                         uint64_t duration);

/**
 * Subscribe to multiple message types of one symbol.
 *
 * This API is suitable for client applications such as APP, website, etc.
 *
 * String messages in `tx` are already parsed by `crypto-msg-parser`.
 */
void subscribe_symbol(const char *exchange,
                      MarketType market_type,
                      const char *symbol,
                      const MessageType *msg_types,
                      unsigned int num_msg_types,
                      void (*on_msg)(const char*),
                      uint64_t duration);

#ifdef __cplusplus
} // extern "C"
#endif // __cplusplus

#endif /* CRYPTO_CRAWLER_H_ */
