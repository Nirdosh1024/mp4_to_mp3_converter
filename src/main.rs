use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};

fn parse_mp4(file_path: &str) -> io::Result<()> {
    let mut file = File::open(file_path)?;

    // Get the total file size
    let file_size = file.metadata()?.len();

    let mut buffer = [0u8; 8]; // Atom header is 8 bytes (size + type)
    let mut offset = 0;

    let mut audio_found = false;

    // Parse the MP4 file
    while offset < file_size {
        if file_size - offset < 8 {
            println!("Incomplete atom header at offset {}", offset);
            break;
        }

        // Read the next 8 bytes (atom header)
        file.read_exact(&mut buffer)?;

        // First read the size
        let size = {
            let size_bytes: [u8; 4] = buffer[0..4].try_into().unwrap();
            u32::from_be_bytes(size_bytes)
        };

        // Then read the atom type
        let atom_type = {
            let type_bytes = &buffer[4..8];
            std::str::from_utf8(type_bytes).unwrap_or("Invalid")
        };

        // Check if this is the moov atom (contains track info)
        if atom_type == "moov" {
            println!("Found 'moov' atom at offset {}", offset);

            offset += 8;
            file.seek(SeekFrom::Start(offset))?;

            while offset < file_size {
                if file_size - offset < 8 {
                    break;
                }

                file.read_exact(&mut buffer)?;

                let size = {
                    let size_bytes: [u8; 4] = buffer[0..4].try_into().unwrap();
                    u32::from_be_bytes(size_bytes)
                };

                let atom_type = {
                    let type_bytes = &buffer[4..8];
                    std::str::from_utf8(type_bytes).unwrap_or("Invalid")
                };

                // Look for 'trak' atoms (tracks)
                if atom_type == "trak" {
                    println!("Found 'trak' atom at offset {}", offset);

                    offset += 8;
                    file.seek(SeekFrom::Start(offset))?;

                    while offset < file_size {
                        if file_size - offset < 8 {
                            break;
                        }

                        file.read_exact(&mut buffer)?;

                        let size = {
                            let size_bytes: [u8; 4] = buffer[0..4].try_into().unwrap();
                            u32::from_be_bytes(size_bytes)
                        };

                        let atom_type = {
                            let type_bytes = &buffer[4..8];
                            std::str::from_utf8(type_bytes).unwrap_or("Invalid")
                        };

                        if atom_type == "mdia" {
                            println!("Found 'mdia' atom at offset {}", offset);

                            offset += 8;
                            file.seek(SeekFrom::Start(offset))?;

                            while offset < file_size {
                                if file_size - offset < 8 {
                                    break;
                                }

                                file.read_exact(&mut buffer)?;

                                let size = {
                                    let size_bytes: [u8; 4] = buffer[0..4].try_into().unwrap();
                                    u32::from_be_bytes(size_bytes)
                                };

                                let atom_type = {
                                    let type_bytes = &buffer[4..8];
                                    std::str::from_utf8(type_bytes).unwrap_or("Invalid")
                                };

                                if atom_type == "hdlr" {
                                    println!("Found 'hdlr' atom at offset {}", offset);

                                    let mut handler_type = [0u8; 4];
                                    file.read_exact(&mut handler_type)?;

                                    let handler_str = std::str::from_utf8(&handler_type).unwrap();

                                    if handler_str == "soun" {
                                        println!("Audio track found at offset {}", offset);
                                        audio_found = true;
                                    }

                                    break;
                                }

                                offset += size as u64;
                                file.seek(SeekFrom::Start(offset))?;
                            }
                        }

                        offset += size as u64;
                        file.seek(SeekFrom::Start(offset))?;
                    }
                }

                offset += size as u64;
                file.seek(SeekFrom::Start(offset))?;
            }
        }

        offset += size as u64;
        file.seek(SeekFrom::Start(offset))?;
    }

    if audio_found {
        println!("Audio track identified successfully!");
    } else {
        println!("No audio track found in the MP4 file.");
    }

    Ok(())
}

fn main() {
    let file_path = "video.mp4"; // Replace with your MP4 file path
    if let Err(e) = parse_mp4(file_path) {
        eprintln!("Error parsing MP4: {}", e);
    }
}
