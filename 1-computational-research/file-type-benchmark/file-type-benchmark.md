## Problem

- 리서치를 하기 위해서는 대용량의 csv 파일(일일 오더북 데이터)들을 읽고 활용하는 과정이 필요함
- 문제는 csv파일들의 크기가 상당히 큼(csv.gz로 4.6 테라바이트) → 로드하는데 시간이 오래걸림
- 로드시간을 줄이기 위해서는 파일을 바이너리로 변환해야 하는데, 바이너리로 변환할 경우 약 50테라바이트 이상의 저장공간이 필요함
- 따라서, 바이너리를 압축하여 보관하되, 빠르게 압축을 풀고 로드할 수 있는 압축형식을 찾고자 벤치마킹을 수행함.

- 참고로, 로드시간을 줄이는 데에는 다음과 같은 방법이 추가로 있음
    - 더 빠른 저장장치 사용
    - 더 빠른 버스 프로토콜 사용
    - 더 빠른 파일시스템 사용

## Benchmark

- 벤치마킹에 사용된 비교대상
    - csv.gz
    - bin
    - bin.zst (최고압축레벨)
        - zstd 압축은 압축 레벨에 따라 압축 시간이 달라지지만 압축 해제레벨은 비슷하다.
    - lz4

## How to test
1. convert csv.gz to other type using 'cargo run --bin convert_all_formats'
2. try to run a benchmark using 'cargo bench'
3. open index.html in target/criterion/report

## Results
Format Time Size (MB) Speed vs CSV Compression Ratio

| Format  | Time  | Size (MB) |
| --- | --- | --- |
| Binary  | 1.77 s | 1196 MB |
| ZST-1 | 3.69 s | 130 MB |
| ZST-3 | 3.68 s | 118 MB |
| ZST-6 | 3.49 s | 108 MB |
| LZ4 | 3.52 s | 218 MB |
| Snappy | 2.89 s | 222 MB |
| csv.gz | 4.23 s | 73 MB |