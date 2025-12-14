# hftbacktest 주문 및 자산 관리

이 문서는 `hftbacktest` 라이브러리를 사용하여 주문을 관리하고 자산 정보를 확인하는 방법에 대한 주요 개념을 설명합니다.

## 1. 자산 (Position)

현재 보유 자산에 대한 정보는 `hbt.position`을 통해 접근할 수 있습니다. 이 메서드는 특정 자산의 현재 포지션 정보를 담고 있는 객체를 반환합니다.

- **변수 접근**: `hbt.position(asset_no)`
- **데이터 타입**: `Position` 객체
- **asset_no**: 자산의 인덱스 (예: 0)

### Position 객체 구조

`Position` 객체는 다음의 주요 속성을 포함합니다:

- `.qty` (float): 현재 보유 수량. 양수이면 롱 포지션, 음수이면 숏 포지션입니다.
- `.balance` (float): 기준 통화(quote currency)의 잔액.
- `.trading_volume` (float): 누적 거래량.
- `.trading_value` (float): 누적 거래 가치.
- `.realized_pnl` (float): 실현 손익.
- `.unrealized_pnl` (float): 미실현 손익.

### 사용 예시

```python
from numba import njit
import numpy as np

@njit
def strategy(hbt):
    # 자산 0의 포지션 정보 가져오기
    position = hbt.position(0)

    # 현재 수량 및 미실현 손익 출력
    print(f"현재 수량: {position.qty}")
    print(f"미실현 손익: {position.unrealized_pnl}")

    # 모든 자산의 포지션 정보를 순회하며 출력
    for i in range(hbt.num_assets):
        position = hbt.position(i)
        print(f"자산 {i}의 수량: {position.qty}")

```

## 2. 주문 실행 및 관리

### 주문 제출 (Submit)

- **매수 주문**: `hbt.submit_buy_order(asset_no, order_id, price, qty, time_in_force, order_type, wait)`
- **매도 주문**: `hbt.submit_sell_order(asset_no, order_id, price, qty, time_in_force, order_type, wait)`

**주요 파라미터:**
- `asset_no` (int): 자산 인덱스
- `order_id` (int): 주문의 고유 ID
- `price` (float): 주문 가격
- `qty` (float): 주문 수량
- `time_in_force` (const): 주문 유효 기간 (`GTC`, `IOC`, `FOK`)
- `order_type` (const): `LIMIT` (지정가) 또는 `MARKET` (시장가)
- `wait` (bool): 주문 응답을 동기적으로 기다릴지 여부

**예시:**
```python
from hftbacktest import GTC, LIMIT

# 1 BTC 지정가 매수
hbt.submit_buy_order(0, 1, 61000.5, 1, GTC, LIMIT, True)

# 0.5 BTC 지정가 매도
hbt.submit_sell_order(0, 2, 61500.0, 0.5, GTC, LIMIT, True)
```

### 주문 확인 (Check)

`hbt.orders(asset_no)`를 사용하여 특정 자산의 모든 활성 및 완료된 주문 목록을 가져올 수 있습니다. 각 주문은 `Order` 객체로 표현됩니다.

```python
# 자산 0의 모든 주문 정보 출력
orders = hbt.orders(0)
order_values = orders.values()
while order_values.has_next():
    order = order_values.get()
    print(
        f"ID: {order.order_id}, "
        f"상태: {order.status}, "
        f"가격: {order.price}, "
        f"수량: {order.qty}"
    )
```

**`Order` 객체 주요 속성:**
- `.status`: `NEW`, `FILLED`, `CANCELED`, `EXPIRED` 등 주문의 현재 상태
- `.req`: `NEW`, `CANCELED` 등 주문에 대한 요청 상태

### 주문 취소 (Cancel)

`hbt.cancel(asset_no, order_id, wait)`를 사용하여 대기 중인 주문을 취소합니다.

**예시:**
```python
# 주문 ID가 1인 주문 취소
hbt.cancel(0, 1, True)
```

## 3. 주요 기능 요약

| 기능 | 메서드 | 설명 |
| :--- | :--- | :--- |
| **포지션 확인** | `hbt.position(asset_no)` | 현재 보유 수량, 잔고, 손익 등 조회 |
| **매수 주문** | `hbt.submit_buy_order(...)` | 지정가 또는 시장가 매수 주문 |
| **매도 주문** | `hbt.submit_sell_order(...)` | 지정가 또는 시장가 매도 주문 |
| **주문 취소** | `hbt.cancel(order_id)` | 대기 중인 주문 취소 |
| **모든 주문 조회** | `hbt.orders(asset_no)` | 특정 자산의 모든 주문 목록 반환 |
| **호가창 조회** | `hbt.depth(asset_no)` | Best bid/ask 등 호가창 정보 조회 |
| **시간 진행** | `hbt.elapse(duration)` | 백테스트 시뮬레이션 시간 진행 |
| **백테스트 종료**| `hbt.close()` | 백테스트 리소스 정리 및 결과 반환 |
