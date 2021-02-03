use std::io::prelude::*;
use std::io::Cursor;
use byteorder::{BigEndian, LittleEndian, ReadBytesExt};

fn main() {
  let mut arguments = std::env::args().skip(1);
  let filename  = match arguments.next() {
    Some(c) => { c },
    None => { return println!("No command supplied") }
  };

  if std::path::Path::new(&filename).exists() {
    let mut file = std::fs::File::open(&filename).unwrap();

    let chunk_id = Chunk::new(4, Endian::Big).read(&mut file);
    let chunk_size = Chunk::new(4, Endian::Little).read(&mut file);
    let chunk_format = Chunk::new(4, Endian::Big).read(&mut file);

    let first_sub_id = Chunk::new(4, Endian::Big).read(&mut file);
    let first_sub_size = Chunk::new(4, Endian::Little).read(&mut file);

    let audio_format = Chunk::new(2, Endian::Little).read(&mut file);
    let isPCM: bool = audio_format == 1;

    let num_channels = Chunk::new(2, Endian::Little).read(&mut file);
    let sample_rate = Chunk::new(4, Endian::Little).read(&mut file);
    let byte_rate = Chunk::new(4, Endian::Little).read(&mut file);
    let block_align = Chunk::new(2, Endian::Little).read(&mut file);
    let bits_per_sample = Chunk::new(2, Endian::Little).read(&mut file);
    // TODO: Check for non-PCM params

    let second_sub_id = Chunk::new(4, Endian::Big).read(&mut file);
    let second_sub_size = Chunk::new(4, Endian::Little).read(&mut file);

    let mut sample_count = second_sub_size * 8 / bits_per_sample;

    println!("Audio format: {}", &audio_format);
    println!("Num channels: {}", &num_channels);
    println!("Sample rate: {}Hz", &sample_rate);
    println!("Bits per sample: {}", &bits_per_sample);
    println!("Byte rate: {}", &byte_rate);
    println!("Subchunk#2 size: {}", &second_sub_size);
    println!("Total samples: {}", &sample_count);

    // Reading the data
    // sample_count = 10;
    let mut buffer: Vec<u32> = vec![0; sample_count as usize];
    for _x in 0..(sample_count - 1) {
      let sample = Chunk::new(bits_per_sample as usize, Endian::Little).read(&mut file);
      println!("{}", sample); // x: i32
    }

    // Reference values
    // 21.3623, -15.2588,  12.2070, -18.3105,   3.0518, -15.2588,  12.2070, -15.2588,   6.1035, -15.2588
  } else {
    println!("No such file found");
  }
}

fn to_u32(slice: &[u8]) -> u32 {
  // Add .rev() after iter to use big endian
  slice.iter().rev().fold(0, |acc, &b| acc * 2 + b as u32)
}

fn print(prefix: &str, vec: &Vec<u8>) {
  match std::str::from_utf8(&vec) {
    Ok(c) => { println!("{}: {}", prefix, c) },
    Err(_e ) => { printvec(&vec) }
  }
}

fn printvec(vec: &Vec<u8>) {
  println!();
  for &byte in vec {
    print!("{}.", byte);
  }
  println!();
}

enum Endian {
  Little,
  Big
}

struct Chunk {
  size: usize,
  endian: Endian
}

impl Chunk {
  fn new(size: usize, endian: Endian) -> Chunk {
    Chunk { size: size, endian: endian }
  }

  fn read(&self, file: &mut std::fs::File) -> u32 {
    let mut buffer: Vec<u8> = vec![0; self.size];
    file.read(&mut buffer);
    let mut rdr = Cursor::new(buffer);

    if self.size == 4 {
      match self.endian {
        Endian::Big => {
          rdr.read_u32::<BigEndian>().unwrap()
        },
        Endian::Little => {
          rdr.read_u32::<LittleEndian>().unwrap()
        }
      }
    } else {
      rdr.read_u8().unwrap() as u32
    }
  }
}
