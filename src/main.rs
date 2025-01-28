use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};

fn parse_mp4(file_path: &str) -> io::Result<()> {
    let mut file = File::open(file_path)?;

    // Get the total file size
    let file_size = file.metadata()?.len();

    let mut buffer = [0u8; 8]; // Atom header is 8 bytes (size + type)

    println!("MP4 Atom Structure:");
    println!("{:<10} {:<10} {:<10}", "Offset", "Type", "Size");

    let mut offset = 0;

    while offset < file_size {
        // Check if we have enough remaining bytes to read the header
        if file_size - offset < 8 {
            println!("Incomplete atom header at offset {}", offset);
            break;
        }

        // Read the next 8 bytes (atom header)
        file.read_exact(&mut buffer)?;

        // Parse the size (first 4 bytes)
        let size = u32::from_be_bytes(buffer[0..4].try_into().unwrap());

        // Parse the type (next 4 bytes)
        let atom_type = std::str::from_utf8(&buffer[4..8]).unwrap_or("Invalid");

        println!("{:<10} {:<10} {:<10}", offset, atom_type, size);

        // If the atom size is invalid, stop processing
        if size < 8 {
            println!("Invalid atom size {} at offset {}", size, offset);
            break;
        }

        // Move the offset forward by the atom size
        offset += size as u64;

        // Seek to the next atom's position
        file.seek(SeekFrom::Start(offset))?;
    }

    Ok(())
}

fn main() {
    let file_path = "video.mp4"; // Replace with your MP4 file path
    if let Err(e) = parse_mp4(file_path) {
        eprintln!("Error parsing MP4: {}", e);
    }
}
