/*code for transform .csv.gz to .bin and .bin.zst*/

use std::fs::File;
use std::io::{BufWriter, Write, Read};
use std::time::Instant;
use anyhow::Result;
use flate2::read::GzDecoder;
use polars::prelude::*;
use bincode;
use zstd::stream::write::Encoder as ZstdEncoder;
use apache_avro::{Writer, Schema};

fn main() -> Result<()> {
    let input_file = "./sample_data/2024-06-26_BTCUSDT.csv.gz";
    
    println!("🔄 CSV.GZ to All Formats Converter");
    println!("===================================");
    
    // Read and parse CSV.GZ
    println!("📖 Reading CSV.GZ file...");
    let start_time = Instant::now();
    
    let file = File::open(input_file)?;
    let mut decoder = GzDecoder::new(file);
    let mut csv_content = String::new();
    decoder.read_to_string(&mut csv_content)?;
    
    let cursor = std::io::Cursor::new(csv_content);
    let df = CsvReader::new(cursor).finish()?;
    
    println!("✅ DataFrame loaded: {:?} in {:?}", df.shape(), start_time.elapsed());
    
    // Serialize once for all compression methods
    println!("🔧 Serializing DataFrame...");
    let serialize_start = Instant::now();
    let df_bytes = bincode::serialize(&df)?;
    println!("✅ Serialized: {} bytes in {:?}", df_bytes.len(), serialize_start.elapsed());
    
    // Get original file size for comparison
    let original_size = std::fs::metadata(input_file)?.len();
    
    println!("\n🗜️  Converting to all formats:");
    println!("==============================");
    
    // Convert to all formats
    let mut results = Vec::new();
    
    let formats = [
        ("Binary (uncompressed)", "./sample_data/2024-06-26_BTCUSDT.bin"),
        ("ZST Level 1 (fastest)", "./sample_data/2024-06-26_BTCUSDT_zst1.bin.zst"),
        ("ZST Level 3 (balanced)", "./sample_data/2024-06-26_BTCUSDT_zst3.bin.zst"),
        ("ZST Level 6 (best ratio)", "./sample_data/2024-06-26_BTCUSDT_zst6.bin.zst"),
        ("LZ4 (fast)", "./sample_data/2024-06-26_BTCUSDT.bin.lz4"),
        ("Snappy (balanced)", "./sample_data/2024-06-26_BTCUSDT.bin.snap"),
    ];
    
    for (name, output_path) in formats {
        print!("  {} ... ", name);
        std::io::stdout().flush().unwrap();
        
        let start = Instant::now();
        
        match name {
            "Binary (uncompressed)" => convert_to_binary(&df_bytes, output_path)?,
            "ZST Level 1 (fastest)" => convert_to_zst(&df_bytes, output_path, 1)?,
            "ZST Level 3 (balanced)" => convert_to_zst(&df_bytes, output_path, 3)?,
            "ZST Level 6 (best ratio)" => convert_to_zst(&df_bytes, output_path, 6)?,
            "LZ4 (fast)" => convert_to_lz4(&df_bytes, output_path)?,
            "Snappy (balanced)" => convert_to_snappy(&df_bytes, output_path)?,
            _ => unreachable!(),
        }
        
        let duration = start.elapsed();
        
        let file_size = std::fs::metadata(output_path)?.len();
        let compression_ratio = df_bytes.len() as f64 / file_size as f64;
        
        println!("✅ {:.1} MB ({:.2}x) in {:?}", 
            file_size as f64 / 1024.0 / 1024.0, 
            compression_ratio, 
            duration
        );
        
        results.push((name, file_size, compression_ratio, duration));
    }
    
    // Print summary table
    println!("\n📊 Conversion Results Summary:");
    println!("==============================");
    println!("{:<25} {:>12} {:>15} {:>12} {:>15}", "Format", "Size (MB)", "Compression", "Time", "vs Original");
    println!("{}", "-".repeat(80));
    
    // Add original file to comparison
    println!("{:<25} {:>9.1} MB {:>12.2}x {:>9} {:>12}", 
        "CSV.GZ (original)", 
        original_size as f64 / 1024.0 / 1024.0,
        df_bytes.len() as f64 / original_size as f64,
        "N/A",
        "1.00x"
    );
    
    for (name, size, compression, time) in &results {
        let vs_original = *size as f64 / original_size as f64;
        println!("{:<25} {:>9.1} MB {:>12.2}x {:>9.2?} {:>12.2}x", 
            name, 
            *size as f64 / 1024.0 / 1024.0,
            compression,
            time,
            vs_original
        );
    }
    
    println!("\n🎯 Recommendations:");
    println!("===================");
    println!("🚀 Fastest reading: Binary (uncompressed)");
    println!("⚖️  Best balance: Snappy or LZ4");
    println!("🗜️  Best compression: ZST Level 6");
    println!("🔄 CSV.GZ replacement: ZST Level 1 or Snappy");
    
    println!("\n✅ All conversions completed successfully!");
    Ok(())
}

fn convert_to_binary(data: &[u8], output_path: &str) -> Result<()> {
    let file = File::create(output_path)?;
    let mut writer = BufWriter::new(file);
    writer.write_all(data)?;
    writer.flush()?;
    Ok(())
}

fn convert_to_zst(data: &[u8], output_path: &str, level: i32) -> Result<()> {
    let file = File::create(output_path)?;
    let mut encoder = ZstdEncoder::new(file, level)?;
    encoder.write_all(data)?;
    encoder.finish()?;
    Ok(())
}

fn convert_to_lz4(data: &[u8], output_path: &str) -> Result<()> {
    let compressed = lz4_flex::compress_prepend_size(data);
    let mut file = File::create(output_path)?;
    file.write_all(&compressed)?;
    Ok(())
}

fn convert_to_snappy(data: &[u8], output_path: &str) -> Result<()> {
    let compressed = snap::raw::Encoder::new().compress_vec(data)?;
    let mut file = File::create(output_path)?;
    file.write_all(&compressed)?;
    Ok(())
}