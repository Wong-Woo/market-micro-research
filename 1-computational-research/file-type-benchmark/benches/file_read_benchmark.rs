/* code for benchmark */
use std::fs::File;
use std::io::{BufReader, Read};
use flate2::read::GzDecoder;
use polars::prelude::*;
use bincode;
use zstd::stream::read::Decoder as ZstdDecoder;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn read_csv_gz() -> PolarsResult<DataFrame> {
    let file = File::open("./sample_data/2024-06-26_BTCUSDT.csv.gz").unwrap();
    let mut decoder = GzDecoder::new(file);
    let mut csv_content = String::new();
    decoder.read_to_string(&mut csv_content).unwrap();
    
    let cursor = std::io::Cursor::new(csv_content);
    CsvReader::new(cursor).finish()
}

fn read_binary() -> Result<DataFrame, Box<dyn std::error::Error>> {
    let mut file = File::open("./sample_data/2024-06-26_BTCUSDT.bin")?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    
    let df: DataFrame = bincode::deserialize(&buffer)?;
    Ok(df)
}

fn read_zst(level: &str) -> Result<DataFrame, Box<dyn std::error::Error>> {
    let filename = format!("./sample_data/2024-06-26_BTCUSDT_zst{}.bin.zst", level);
    let file = File::open(&filename)?;
    let mut decoder = ZstdDecoder::new(BufReader::new(file))?;
    let mut buffer = Vec::new();
    decoder.read_to_end(&mut buffer)?;
    
    let df: DataFrame = bincode::deserialize(&buffer)?;
    Ok(df)
}

fn read_lz4() -> Result<DataFrame, Box<dyn std::error::Error>> {
    let mut file = File::open("./sample_data/2024-06-26_BTCUSDT.bin.lz4")?;
    let mut compressed = Vec::new();
    file.read_to_end(&mut compressed)?;
    
    let buffer = lz4_flex::decompress_size_prepended(&compressed)?;
    let df: DataFrame = bincode::deserialize(&buffer)?;
    Ok(df)
}

fn read_snappy() -> Result<DataFrame, Box<dyn std::error::Error>> {
    let mut file = File::open("./sample_data/2024-06-26_BTCUSDT.bin.snap")?;
    let mut compressed = Vec::new();
    file.read_to_end(&mut compressed)?;
    
    let buffer = snap::raw::Decoder::new().decompress_vec(&compressed)?;
    let df: DataFrame = bincode::deserialize(&buffer)?;
    Ok(df)
}

fn benchmark_file_formats(c: &mut Criterion) {
    c.bench_function("csv_gz", |b| b.iter(|| black_box(read_csv_gz().unwrap())));
    c.bench_function("binary", |b| b.iter(|| black_box(read_binary().unwrap())));
    c.bench_function("zst_level_1", |b| b.iter(|| black_box(read_zst("1").unwrap())));
    c.bench_function("zst_level_3", |b| b.iter(|| black_box(read_zst("3").unwrap())));
    c.bench_function("zst_level_5", |b| b.iter(|| black_box(read_zst("6").unwrap())));
    c.bench_function("lz4", |b| b.iter(|| black_box(read_lz4().unwrap())));
    c.bench_function("snappy", |b| b.iter(|| black_box(read_snappy().unwrap())));
}

criterion_group!(benches, benchmark_file_formats);
criterion_main!(benches);