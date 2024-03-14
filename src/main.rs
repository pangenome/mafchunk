use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Usage: {} <input_file.maf> <split_length>", args[0]);
        std::process::exit(1);
    }

    let input_path = &args[1];
    let split_length: usize = args[2].parse().expect("Split length must be an integer");

    let file = File::open(input_path)?;
    let reader = BufReader::new(file);

    let mut current_block = Vec::new();
    let mut current_block_size = 0;

    for line in reader.lines() {
        let line = line?;
        if line.starts_with("a") {
            if !current_block.is_empty() {
                emit_blocks(&current_block, current_block_size, split_length)?;
                current_block.clear();
            }
            current_block.push(line);
        } else if line.starts_with("s") {
            current_block.push(line.clone());
            let parts: Vec<&str> = line.split_whitespace().collect();
            let size: usize = parts[3].parse().unwrap();
            if current_block_size == 0 {
                current_block_size = size;
            }
        } else if !line.starts_with("##") {
            current_block.push(line);
        }
    }

    if !current_block.is_empty() {
        emit_blocks(&current_block, current_block_size, split_length)?;
    }

    Ok(())
}

fn emit_blocks(block: &Vec<String>, size: usize, split_length: usize) -> io::Result<()> {
    let splits = (size as f64 / split_length as f64).ceil() as usize;

    for i in 0..splits {
        println!("a");
        for line in block.iter() {
            if line.starts_with("s") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                let src = parts[1];
                let start: usize = parts[2].parse().unwrap();
                //let original_size: usize = parts[3].parse().unwrap();
                let strand = parts[4];
                let src_size = parts[5];
                let seq = parts[6];

                let new_start = start + i * split_length;
                let new_size = if i == splits - 1 { size % split_length } else { split_length };
                let new_seq = &seq[i * split_length..i * split_length + new_size];

                println!(
                    "s {} {} {} {} {} {}",
                    src, new_start, new_size, strand, src_size, new_seq
                );
            } else {
                println!("{}", line);
            }
        }
    }

    Ok(())
}
