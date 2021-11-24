#include <iostream>

#include "crypto_crawler.h"

void on_msg(const char *msg) { std::cout << msg << std::endl; }

int main() {
  MessageType msg_types[2] = {MessageType::Trade, MessageType::L2Event};
  subscribe_symbol("binance", MarketType::LinearSwap, "BTCUSDT", msg_types, 2,
                   on_msg, 0);
}
