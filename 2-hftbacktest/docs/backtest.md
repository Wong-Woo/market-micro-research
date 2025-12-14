# 백테스트 원리

## HFT 백테스트란?

과거 시장 데이터를 사용하여 **틱(tick) 단위**로 오더북을 재구성하고, 주문의 체결 과정을 시뮬레이션하는 것입니다. 일반 백테스트와 달리, HFT 백테스트는 다음을 정밀하게 시뮬레이션합니다:

- 오더북의 실시간 변화 (호가 추가/삭제)
- 주문이 호가창에서 대기하는 **큐(Queue) 위치**
- 주문 전송 및 응답 **레이턴시(Latency)**
- **부분 체결(Partial Fill)** 가능성

## 3가지 핵심 메커니즘

### 1. Snapshot (초기 오더북 생성)
**$T=0$ 시점에 오더북의 초기 상태를 설정**

```
시간: 00:00:00.000
┌─────────────────────┐
│   Ask (매도 호가)    │
├─────────────────────┤
│ $61,010  →  0.5 BTC │
│ $61,005  →  1.2 BTC │
│ $61,000  →  2.0 BTC │ ← Best Ask
├─────────────────────┤
│ $60,995  →  1.8 BTC │ ← Best Bid
│ $60,990  →  1.5 BTC │
│ $60,985  →  0.8 BTC │
├─────────────────────┤
│   Bid (매수 호가)    │
└─────────────────────┘
```

### 2. Incremental (호가 변경 반영)
**$T>0$ 이후, 실시간으로 오더북 업데이트**

```
이벤트 1 (T=100ms): 새로운 매도 호가 추가
  → $61,000에 0.3 BTC 추가
  → 호가창: $61,000 → 2.0 + 0.3 = 2.3 BTC

이벤트 2 (T=150ms): 매수 호가 취소
  → $60,995에서 0.5 BTC 제거
  → 호가창: $60,995 → 1.8 - 0.5 = 1.3 BTC

이벤트 3 (T=200ms): 시장가 매수 체결
  → Best Ask($61,000)에서 1.0 BTC 체결
  → 호가창: $61,000 → 2.3 - 1.0 = 1.3 BTC
```

### 3. Trades (내 주문의 큐 관리 및 체결)
**호가창에 제출한 내 주문의 대기 순서와 체결 시뮬레이션**

```
시나리오: $61,000에 매도 지정가 주문 0.5 BTC 제출

초기 상태:
  $61,000 호가창: 2.0 BTC 대기 중
  내 주문: 2.0 BTC 뒤에 대기 (큐 포지션 = 2.0)

이벤트 1: 0.8 BTC 시장가 매수 체결
  → 큐 포지션: 2.0 - 0.8 = 1.2 BTC

이벤트 2: 1.2 BTC 시장가 매수 체결
  → 큐 포지션: 1.2 - 1.2 = 0
  → ✅ 내 주문 체결 시작

이벤트 3: 0.5 BTC 시장가 매수 체결
  → ✅ 내 주문 0.5 BTC 전량 체결 완료
```

---

# HFTBacktest Asset 설정 메서드

## 1. 데이터 설정

### `.data(data)`
백테스트에 사용할 피드 데이터 설정

**Parameters:**
- `data` (str | List[str] | ndarray): 파일 경로 또는 numpy 배열

**Example:**
```python
asset.data(['BTCUSDT_20240626.npz'])
```

### `.initial_snapshot(snapshot)`
백테스트 시작 시점의 초기 오더북 스냅샷 설정

**Parameters:**
- `snapshot` (str | ndarray): 초기 스냅샷 파일 경로 또는 numpy 배열

**Example:**
```python
asset.initial_snapshot('BTCUSDT_20240626.npz')
```

---

## 2. 자산 타입

### `.linear_asset(contract_size)`
선형 계약 자산 설정 (USDT-margined futures, BTC-marin 거래소에서 백테스트시 inverse_asset을 사용하는데 지금은 거의 사용하지 않음)

**Parameters:**
- `contract_size` (float): 계약 크기 (1계약당 거래 수량, 거래소마다 수량 다를 수 있음. 일반적으로는 1임)

**Example:**
```python
asset.linear_asset(1.0)
```

**손익 계산:**
```
PnL = (exit_price - entry_price) × qty × contract_size
```

---

## 3. 레이턴시 설정

### `.constant_order_latency(entry_latency, resp_latency)`
주문 전송 및 응답 레이턴시를 상수로 설정

**Parameters:**
- `entry_latency` (int64): 주문이 거래소에 도달하는 시간 (nanoseconds)
- `resp_latency` (int64): 거래소 응답이 돌아오는 시간 (nanoseconds)

**시간 단위:**
```
1 millisecond = 1,000,000 ns = 1e6 ns
10 millisecond = 10,000,000 ns
1 second = 1,000,000,000 ns = 1e9 ns
```

**Example:**
```python
# 주문 전송 10ms, 응답 10ms
asset.constant_order_latency(10_000_000, 10_000_000)
```

---

## 4. 큐 포지션 모델

큐 모델은 호가창에 제출된 주문이 언제 체결될지 예측합니다.

### `.risk_adverse_queue_model()`
가장 보수적인 큐 모델 (실제 거래 발생 시에만 큐 위치 전진)

**Example:**
```python
asset.risk_adverse_queue_model()
```

### `.power_prob_queue_model(n)`
확률 기반 큐 모델

취소주문이 발생했을 때, 내 주문의 위치를 예측하는 확률모델

**Parameters:**
- `n` (float): 거듭제곱 지수
  - `n = 1`: 낙관적 (빠른 체결)
  - `n = 2`: 중립적 (일반적)
  - `n = 3`: 보수적 (느린 체결)

**Example:**
```python
# 일반적인 설정
asset.power_prob_queue_model(2.0)
```

**큐 모델 선택 가이드:**
- 보수적 백테스트: `risk_adverse_queue_model()` 또는 `power_prob_queue_model(3.0)`
- 일반적인 경우: `power_prob_queue_model(2.0)`

---

## 5. 체결 모델

### `.no_partial_fill_exchange()`
부분 체결 불허 (전량 체결 또는 미체결)

**Example:**
```python
asset.no_partial_fill_exchange()
```

### `.partial_fill_exchange()`
부분 체결 허용 (실제 거래소와 동일)

**Example:**
```python
asset.partial_fill_exchange()
```

---

## 6. 수수료 설정

### `.trading_value_fee_model(maker_fee, taker_fee)`
거래 가치 기반 수수료 (암호화폐 거래소 방식)

**Parameters:**
- `maker_fee` (float): Maker 수수료율 (예: 0.0002 = 0.02%)
- `taker_fee` (float): Taker 수수료율 (예: 0.0007 = 0.07%)

**수수료 계산:**
```
Fee = abs(price × qty) × fee_rate
```

**Example:**
```python
# Binance Futures 기본 수수료
asset.trading_value_fee_model(0.0002, 0.0007)
```

**Maker vs Taker:**
- **Maker**: 호가창에 유동성 제공 (지정가 주문 대기)
- **Taker**: 호가창 유동성 소비 (즉시 체결)

---

## 7. 자산 스펙

### `.tick_size(size)`
최소 가격 단위

**Parameters:**
- `size` (float): 최소 가격 변동 단위

**Example:**
```python
asset.tick_size(0.1)  # BTCUSDT: 0.1 USDT
```

### `.lot_size(size)`
최소 주문 수량 단위

**Parameters:**
- `size` (float): 최소 주문 수량

**Example:**
```python
asset.lot_size(0.001)  # BTCUSDT: 0.001 BTC
```

---

# 백테스트 실행 프로세스

## 1. Asset 설정

```python
from hftbacktest import BacktestAsset, HashMapMarketDepthBacktest

asset = (
    BacktestAsset()
    .data(['BTCUSDT_20240626.npz'])
    .initial_snapshot('BTCUSDT_20240626.npz')
    .linear_asset(1.0)
    .constant_order_latency(10_000_000, 10_000_000)
    .risk_adverse_queue_model()
    .no_partial_fill_exchange()
    .trading_value_fee_model(0.0002, 0.0007)
    .tick_size(0.1)
    .lot_size(0.001)
)
```

## 2. 백테스트 인스턴스 생성

```python
hbt = HashMapMarketDepthBacktest([asset])
```

## 3. 전략 구현 및 실행

```python
from numba import njit
import numpy as np

@njit
def backtest_strategy(hbt):
    while hbt.elapse(60 * 1e9) == 0:  # 60초씩 전진
        depth = hbt.depth(0)

        best_bid = depth.best_bid
        best_ask = depth.best_ask

        # 전략 로직
        # - 주문: hbt.submit_buy_order() / hbt.submit_sell_order()
        # - 취소: hbt.cancel()

    return True

backtest_strategy(hbt)
_ = hbt.close()
```

---

# 주요 백테스트 메서드

## `hbt.elapse(duration)`
백테스트 시간 전진

**Parameters:**
- `duration` (int64): 전진할 시간 (nanoseconds)

**Returns:**
- `0`: 정상, `1`: 데이터 종료

**Example:**
```python
hbt.elapse(60 * 1e9)  # 60초 전진
```

## `hbt.depth(asset_no)`
현재 오더북 깊이 정보

**Parameters:**
- `asset_no` (uint64): 자산 인덱스 (0부터 시작)

**Returns:**
- `best_bid` (float64): 최우선 매수 호가
- `best_ask` (float64): 최우선 매도 호가

**Example:**
```python
depth = hbt.depth(0)
spread = depth.best_ask - depth.best_bid
```

## `hbt.submit_buy_order(asset_no, order_id, price, qty, time_in_force, order_type, wait)`
매수 주문 제출

**Parameters:**
- `asset_no` (uint64): 자산 인덱스
- `order_id` (uint64): 주문 ID (고유값)
- `price` (float64): 주문 가격
- `qty` (float64): 주문 수량
- `time_in_force` (uint8): 주문 유효 기간
  - `GTC`: Good-Till-Cancel (체결/취소 시까지)
  - `GTX`: Post-only (Maker만 허용)
  - `IOC`: Immediate-Or-Cancel (즉시 체결 가능한 수량만)
  - `FOK`: Fill-Or-Kill (전량 즉시 체결 또는 전량 취소)
- `order_type` (uint8): `LIMIT` (지정가) 또는 `MARKET` (시장가)
- `wait` (bool): 응답 대기 여부

**Example:**
```python
from hftbacktest import GTC, LIMIT

hbt.submit_buy_order(
    asset_no=0,
    order_id=1,
    price=61000.0,
    qty=0.1,
    time_in_force=GTC,
    order_type=LIMIT,
    wait=True
)
```

### Time-In-Force 옵션

| 옵션 | 의미 | 사용 시나리오 |
|------|------|---------------|
| GTC | 체결/취소 시까지 유지 | 일반적인 지정가 주문 |
| GTX | Taker 체결 방지 | Maker 수수료만 원할 때 |
| IOC | 즉시 체결 가능한 수량만 | 부분 체결 허용 |
| FOK | 전량 즉시 체결 or 취소 | All-or-Nothing |

## `hbt.submit_sell_order(...)`
매도 주문 제출 (파라미터는 submit_buy_order와 동일)

**Example:**
```python
hbt.submit_sell_order(0, 2, 61100.0, 0.1, GTC, LIMIT, True)
```

## `hbt.cancel(asset_no, order_id, wait)`
주문 취소

**Parameters:**
- `asset_no` (uint64): 자산 인덱스
- `order_id` (uint64): 취소할 주문 ID
- `wait` (bool): 응답 대기 여부

**Example:**
```python
hbt.cancel(0, 1, True)
```

---

# 백테스트 워크플로우

1. **데이터 준비**: 거래소 데이터를 .npz 형식으로 변환
2. **Asset 설정**: BacktestAsset으로 설정
3. **백테스트 생성**: HashMapMarketDepthBacktest 인스턴스 생성
4. **전략 구현**: @njit 데코레이터 사용
5. **실행 및 분석**: 전략 실행 후 결과 분석

**Note:**
- `@njit` 사용으로 백테스트 속도 향상
- 백테스트 인스턴스는 재사용 불가

---

# 참고 자료

- [hftbacktest 공식 문서](https://hftbacktest.readthedocs.io/en/latest/)
- [GitHub Repository](https://github.com/nkaz001/hftbacktest)
