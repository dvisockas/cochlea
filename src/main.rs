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


    // Reading the formatting
    let mut fmt = vec![0; 4];
    let mut subchunk_size = vec![0; 4];
    let mut audio_format = vec![0; 2];
    let mut num_channels = vec![0; 2];
    let mut sample_rate = vec![0; 4];

    file.read(&mut fmt);
    file.read(&mut subchunk_size);
    // chunk_size.reverse();
    file.read(&mut audio_format);
    file.read(&mut num_channels);
    file.read(&mut sample_rate);

    let mut rdr = Cursor::new(sample_rate);
    let sample_rate = rdr.read_u32::<LittleEndian>().unwrap();

    // sample_rate.reverse();
    // num_channels.reverse();

    println!("Num channels: {}", to_u32(&num_channels));
    print("fmt", &fmt);
    print!("Sample rate: {}Hz", &sample_rate);

    // Reading the data

  } else {
    println!("No such file found");
  }

  println!()
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

struct Header {
  // chunks: [Chunk; 5]
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
