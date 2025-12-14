from numba import njit
import numpy as np

from hftbacktest import BacktestAsset, HashMapMarketDepthBacktest

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


_ = hbt.close()