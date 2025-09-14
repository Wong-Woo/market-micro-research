use std::fs::File;
use std::io::{Read, Write};
use flate2::read::GzDecoder;
use polars::prelude::*;

fn save_dataframe_to_binary(df: &DataFrame, path: &str) -> anyhow::Result<()> {
    let serialized = bincode::serialize(df)?;
    let mut file = File::create(path)?;
    file.write_all(&serialized)?;
    println!("DataFrame saved to binary file: {}", path);
    Ok(())
}

fn read_dataframe_from_binary(path: &str) -> anyhow::Result<DataFrame> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    let df: DataFrame = bincode::deserialize(&buffer)?;
    println!("DataFrame loaded from binary file: {}", path);
    Ok(df)
}

fn main() -> anyhow::Result<()> {
    // 1. gz 파일 열기
    let file = File::open("./sample_data/2024-06-26_BTCUSDT.csv.gz")?;
    let mut decoder = GzDecoder::new(file);
    
    // 2. 압축 해제된 내용을 메모리로 읽기
    let mut csv_content = String::new();
    decoder.read_to_string(&mut csv_content)?;

    // 3. CsvReader로 읽어서 DataFrame으로 변환
    let cursor = std::io::Cursor::new(csv_content);
    let df: DataFrame = CsvReader::new(cursor)
        .finish()?;

    // 4. 원본 데이터 확인
    println!("Original DataFrame:");
    println!("{:?}", df.head(Some(5)));
    
    // 5. DataFrame을 바이너리 파일로 저장
    let binary_path = "./sample_data/btc_data.bin";
    save_dataframe_to_binary(&df, binary_path)?;

    // 6. 바이너리 파일에서 DataFrame 읽기
    let loaded_df = read_dataframe_from_binary(binary_path)?;
    
    // 7. 로드된 데이터 확인
    println!("\nLoaded DataFrame:");
    println!("{:?}", loaded_df.head(Some(5)));

    Ok(())
}