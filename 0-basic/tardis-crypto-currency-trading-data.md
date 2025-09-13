# Tardis Crypto-Currency Trading Data

Tardis 데이터셋에 대해 설명한다.

## Intro

- Binance, Coinbase, Bybit 등 주요 거래소 raw data 수집
- Tick-by-tick 레벨의 고해상도 데이터
- 2019년부터 축적된 방대한 historical dataset

## Trading Data Tree

/mnt/external-hdd/Tardis/
└── binance-futures
    ├── **book_snapshot_25 (25레벨 호가창 스냅샷)**
    │   ├── ...
    ├── book_ticker (실시간 BBO 요약 / BBO: Best Bid(매수호가) + Offer(매도호가))
    │   ├── ...
    ├── derivative_ticker (선물 메타데이터)
    │   ├── ...
    ├── **incremental_book_L2 (25~1000레벨 호가창 변화의 델타이벤트)**
    │   └── FUTURES
    ├── liquidations (강제청산 이벤트)
    │   ├── ...
    ├── quotes (BBO 틱 데이터)
    │   ├── ...
    └── trades (실제 체결된 거래)
        ├── ...

## How to reconstruct order book

- L1: quotes (BBO) - 실시간 스트림
- L2: snapshot + incremental 조합
  - book_snapshot_25 (절대값)
  - incremental_book_L2 (증분값)