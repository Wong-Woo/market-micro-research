from numba import njit
import numpy as np

from hftbacktest import BacktestAsset, HashMapMarketDepthBacktest

from numba import njit
import numpy as np
from hftbacktest import BacktestAsset, HashMapMarketDepthBacktest

@njit
def print_3depth(hbt):
    while hbt.elapse(60 * 1e9) == 0:
        print('current_timestamp:', hbt.current_timestamp)

        # Gets the market depth for the first asset, in the same order as when you created the backtest.
        depth = hbt.depth(0)

        # a key of bid_depth or ask_depth is price in ticks.
        # (integer) price_tick = price / tick_size
        i = 0
        for tick_price in range(depth.best_ask_tick, depth.best_ask_tick + 100):
            qty = depth.ask_qty_at_tick(tick_price)
            if qty > 0:
                print(
                    'ask: ',
                    qty,
                    '@',
                    np.round(tick_price * depth.tick_size, 1)
                )

                i += 1
                if i == 3:
                    break
        i = 0
        for tick_price in range(depth.best_bid_tick, max(depth.best_bid_tick - 100, 0), -1):
            qty = depth.bid_qty_at_tick(tick_price)
            if qty > 0:
                print(
                    'bid: ',
                    qty,
                    '@',
                    np.round(tick_price * depth.tick_size, 1)
                )

                i += 1
                if i == 3:
                    break
    return True

asset = (
    BacktestAsset()
    .data(['BTCUSDT_20240626.npz'])
    .initial_snapshot('BTCUSDT_20240626.npz')
    .linear_asset(1.0)
    .constant_order_latency(10_000_000, 10_000_000)
    .no_partial_fill_exchange()
    .trading_value_fee_model(0.0002, 0.0007)
    .tick_size(0.1)
    .lot_size(0.001)
    .last_trades_capacity(0)
)

hbt = HashMapMarketDepthBacktest([asset])

print_3depth(hbt)

_ = hbt.close()