use std::fs::File;
use std::io::prelude::*;

const CHUNK_SIZE: usize = 4096;

fn main() -> std::io::Result<()> {
    let chunk_count = 1024 * 500;
    let mut file = File::create("foo.txt")?;
    let target_size = CHUNK_SIZE * chunk_count;

    for i in 0..chunk_count {
        let chunk : [u8;CHUNK_SIZE] = generate_random_content();
        let result = file.write(&chunk);

        match result {
            Ok(_written_bytes) => {
                if i % 1000 == 0 {
                    let current_position = CHUNK_SIZE * i;
                    let percentage : f32 = (current_position as f32) / (target_size as f32) * (100 as f32);
                    println!("- writing {} of {} bytes ({:.2}%)", current_position, target_size, percentage);
                }
            }
            Err(error) => {
                println!("Error occured {}", error);
            }
        }
    }

    println!("Operation complete!");

    Ok(())
}


fn generate_random_content() -> [u8;CHUNK_SIZE]  {
    let random_bytes: Vec<u8> = (0..CHUNK_SIZE).map(|_| { rand::random::<u8>() }).collect();
    let bytes: [u8;CHUNK_SIZE] = random_bytes.try_into().unwrap();
    return bytes;
}