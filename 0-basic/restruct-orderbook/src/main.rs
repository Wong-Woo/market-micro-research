use std::fs::File;
use std::io::Read;
use flate2::read::GzDecoder;
use polars::prelude::*;

fn main() -> anyhow::Result<()> {
    let directory_location = "/mnt/quant-data/crypto/tardis/binance-futures/book_snapshot_25/BTCUSDT/";
    // 1. gz 파일 열기
    let file = File::open( directory_location.to_string() + "2024-06-26_BTCUSDT.csv.gz")?;
    let mut decoder = GzDecoder::new(file);
    
    // 2. 압축 해제된 내용을 메모리로 읽기
    let mut csv_content = String::new();
    decoder.read_to_string(&mut csv_content)?;

    // 3. CsvReader로 읽어서 DataFrame으로 변환
    let cursor = std::io::Cursor::new(csv_content);
    let df: DataFrame = CsvReader::new(cursor).finish()?;

    // 4. 원본 데이터 확인
    println!("Original DataFrame:");
    println!("{:?}", df.head(Some(5)));

    Ok(())
}