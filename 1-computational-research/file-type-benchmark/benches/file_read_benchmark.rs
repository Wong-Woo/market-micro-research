/* code for benchmark */
use std::fs::File;
use std::io::{BufReader, Read};
use flate2::read::GzDecoder;
use polars::prelude::*;
use bincode;
use zstd::stream::read::Decoder as ZstdDecoder;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn read_csv_gz(file_path: &str) -> PolarsResult<DataFrame> {
    let file = File::open(file_path).unwrap();
    let mut decoder = GzDecoder::new(file);
    let mut csv_content = String::new();
    decoder.read_to_string(&mut csv_content).unwrap();
    
    let cursor = std::io::Cursor::new(csv_content);
    CsvReader::new(cursor).finish()
}

fn read_binary(file_path: &str) -> Result<DataFrame, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    
    let df: DataFrame = bincode::deserialize(&buffer)?;
    Ok(df)
}

fn read_zst(file_path: &str) -> Result<DataFrame, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let mut decoder = ZstdDecoder::new(BufReader::new(file))?;
    let mut buffer = Vec::new();
    decoder.read_to_end(&mut buffer)?;
    
    let df: DataFrame = bincode::deserialize(&buffer)?;
    Ok(df)
}

fn read_lz4(file_path: &str) -> Result<DataFrame, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    let mut compressed = Vec::new();
    file.read_to_end(&mut compressed)?;
    
    let buffer = lz4_flex::decompress_size_prepended(&compressed)?;
    let df: DataFrame = bincode::deserialize(&buffer)?;
    Ok(df)
}

fn read_snappy(file_path: &str) -> Result<DataFrame, Box<dyn std::error::Error>> {
    let mut file = File::open(file_path)?;
    let mut compressed = Vec::new();
    file.read_to_end(&mut compressed)?;
    
    let buffer = snap::raw::Decoder::new().decompress_vec(&compressed)?;
    let df: DataFrame = bincode::deserialize(&buffer)?;
    Ok(df)
}

fn benchmark_file_formats(c: &mut Criterion) {
    c.bench_function("csv_gz", |b| b.iter(|| black_box(read_csv_gz("/mnt/quant-data/crypto/tardis/binance-futures/book_snapshot_25/BTCUSDT/2024-06-26_BTCUSDT.csv.gz").unwrap())));
    c.bench_function("binary", |b| b.iter(|| black_box(read_binary("./sample_data/2024-06-26_BTCUSDT_zst1.bin").unwrap())));
    c.bench_function("zst_level_1", |b| b.iter(|| black_box(read_zst("./sample_data/2024-06-26_BTCUSDT_zst1.bin.zst").unwrap())));
    c.bench_function("zst_level_3", |b| b.iter(|| black_box(read_zst("./sample_data/2024-06-26_BTCUSDT_zst3.bin.zst").unwrap())));
    c.bench_function("zst_level_5", |b| b.iter(|| black_box(read_zst("./sample_data/2024-06-26_BTCUSDT_zst6.bin.zst").unwrap())));
    c.bench_function("lz4", |b| b.iter(|| black_box(read_lz4("./sample_data/2024-06-26_BTCUSDT_zst{}.bin.lz4").unwrap())));
    c.bench_function("snappy", |b| b.iter(|| black_box(read_snappy("./sample_data/2024-06-26_BTCUSDT_zst{}.bin.snap").unwrap())));
}

criterion_group!(benches, benchmark_file_formats);
criterion_main!(benches);