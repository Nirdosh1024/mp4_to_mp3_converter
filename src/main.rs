use std::fs::{write, File};
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::process::Command;

#[derive(Debug)]
struct Mp4Atom {
    offset: u64,
    size: u32,
    atom_type: String,
}

// Parses an Mp4 file and extracts atoms
fn parse_mp4(file_path: &str) -> io::Result<Vec<Mp4Atom>> {
    let mut file = File::open(file_path)?;
    let file_size = file.metadata()?.len();

    let mut atoms = Vec::new();
    let mut buffer = [0u8; 8]; // Atom header is 8 bytes (size + type)
    let mut offset = 0;

    while offset < file_size {
        if file_size - offset < 8 {
            break;
        }

        file.seek(SeekFrom::Start(offset))?;
        file.read_exact(&mut buffer)?;

        let size = u32::from_be_bytes(buffer[0..4].try_into().unwrap());
        let atom_type = std::str::from_utf8(&buffer[4..8])
            .unwrap_or("Invalid")
            .to_string();

        atoms.push(Mp4Atom {
            offset,
            size,
            atom_type,
        });

        offset += if size > 8 { size as u64 } else { 8 }; // Prevent infinite loop on bad data
    }

    Ok(atoms)
}

// Extracts the audio data from 'mdat' atom (Placeholder)
fn extract_audio_data(file_path: &str, atoms: &[Mp4Atom]) -> io::Result<Vec<u8>> {
    let mut file = File::open(file_path)?;
    let mut audio_data = Vec::new();

    for atom in atoms {
        if atom.atom_type == "mdat" {
            file.seek(SeekFrom::Start(atom.offset + 8))?;
            let mut buffer = vec![0u8; (atom.size - 8) as usize];
            file.read_exact(&mut buffer)?;
            audio_data.extend(buffer);
        }
    }

    Ok(audio_data)
}

// Converts AAC to MP3 using FFmpeg
fn convert_aac_to_mp3(aac_data: Vec<u8>) -> Vec<u8> {
    let temp_aac = "audio.aac";
    let temp_mp3 = "output.mp3";

    let mut file = std::fs::File::create(temp_aac).expect("Failed to create AAC file");
    file.write_all(&aac_data).expect("Failed to write aac file");

    let _output = Command::new("ffmpeg")
        .args(["-i", temp_aac, "-q:a", "2", "-y", temp_mp3])
        .output()
        .expect("Failed to execute ffmpeg");

    std::fs::read(temp_mp3).expect("Failed to read MP3 file")
}

fn main() {
    let file_path = "video.mp4";
    if let Ok(atoms) = parse_mp4(file_path) {
        if let Ok(aac_data) = extract_audio_data(file_path, &atoms) {
            let mp3_data = convert_aac_to_mp3(aac_data);

            write("output.mp3", &mp3_data).expect("Failed to write Mp3 file");
            println!("MP3 conversion complete!");
        } else {
            println!("No audio track found.");
        }
    } else {
        println!("Failed to parse MP4 file.");
    }
}
