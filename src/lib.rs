mod config;
mod errors;

use std::error::Error;
use std::iter::Iterator;
use std::path::Path;
use std::ops::Not;
use std::fs::{File, OpenOptions};
use std::io::Write;

use rand::prelude::*;
use rand::distributions::{Slice, Bernoulli};
use rand::Rng;
use rand_xorshift::XorShiftRng;

use crate::config::*;
use crate::errors::*;

pub fn generate_file_name(conf: & RngConfig) -> Result<String, Box<dyn Error>> {
    match conf {
	RngConfig::StringGen {value: f_name} => Ok((*f_name).clone()),
	RngConfig::CycleGen {
		value: name_vec,
		length: name_len
	} => {
	    let name_iter: std::slice::Iter<u8> = (*name_vec).iter();
	    let new_name_vec: Vec<u8> = name_iter.cycle().take(*name_len).copied().collect();
	    Ok(String::from_utf8(new_name_vec)?)
	},
	_ => Err(Box::new(WrongConfigDataError))
    }
}

pub struct GenByteResult {
    byte: u8,
    bit: bool,
}

// TODO test this
fn generate_bit(acc: GenByteResult, bit_change: (usize, bool)) -> GenByteResult {
    let val_bit: bool = if bit_change.1 {acc.bit} else {acc.bit.not()};
    let val: u8 = if val_bit {
	acc.byte + (1 << (7 - bit_change.0))
    } else {
	acc.byte
    };
    GenByteResult {byte: val, bit: val_bit}
}

// TODO test this
pub fn generate_byte(distr: & Bernoulli, rng: & mut impl Rng, init_bit: bool) -> GenByteResult {
    let init_val = GenByteResult {byte: 0u8, bit: init_bit};
    rng.sample_iter(distr).enumerate().take(8).fold(init_val, generate_bit)
}

// TODO test this
pub fn generate_file_content(conf: & RngConfig) -> Result<Vec<u8>, Box<dyn Error>> {
    match conf {
	RngConfig::CycleGen {
	    value: val_vec,
	    length: val_len
	} => {
	    let val_iter: std::slice::Iter<u8> = (*val_vec).iter();
	    let new_val_vec: Vec<u8> = val_iter.cycle().take(*val_len).copied().collect();
	    Ok(new_val_vec)
	},
	RngConfig::RangeGen {
	    value: val_vecs,
	    seed: seed_opt,
	    length: val_len
	} => {
	    let rng: XorShiftRng;
	    if let Some(s) = seed_opt { // TODO Test this
		let mut rng_seed: [u8; 16] = [0; 16];
		let ending_zero: u8 = 0;
		(*s).iter().chain(std::iter::repeat(& ending_zero)).zip(rng_seed.iter_mut()).for_each(|(a, b)| *b = *a);
		rng = XorShiftRng::from_seed(rng_seed);
	    } else {
		rng = XorShiftRng::from_entropy();
	    }
	    let res_dist: Slice<Vec<u8>> = Slice::new(val_vecs)?;
	    let res_vec: Vec<u8> = rng.sample_iter(res_dist).flatten().take(*val_len).copied().collect();
	    Ok(res_vec)
	},
	RngConfig::BitTrain {
	    seed: seed_opt,
	    length: val_len,
	    probability: val_prob,
	} => {
	    let mut rng: XorShiftRng;
	    if let Some(s) = seed_opt { // TODO Test this
		let mut rng_seed: [u8; 16] = [0; 16];
		let ending_zero: u8 = 0;
		(*s).iter().chain(std::iter::repeat(& ending_zero)).zip(rng_seed.iter_mut()).for_each(|(a, b)| *b = *a);
		rng = XorShiftRng::from_seed(rng_seed);
	    } else {
		rng = XorShiftRng::from_entropy();
	    }
	    let res_dist: Bernoulli = Bernoulli::new(*val_prob)?;
	    let mut res_vec: Vec<u8> = vec![0u8; *val_len];
	    let mut start_bit: bool = rng.gen_bool(0.5f64);
	    for res_byte in & mut res_vec {
		let tmp: GenByteResult = generate_byte(& res_dist, & mut rng, start_bit);
		*res_byte = tmp.byte;
		start_bit = tmp.bit;
	    }
	    Ok(res_vec) 
	},
	RngConfig::StringGen {
	    value: val_str,
	} => {
	    Ok((val_str.clone()).into_bytes())
	}
    }
}

// TODO unite generate_file and generate_directory

pub fn generate_file(conf: & ItemConfig, dir_path: & Path) -> Result<(), Box<dyn Error>> {
    if let ItemConfig::FileConfig {name: name_conf, rand_gen: content_conf} = conf {
	let file_name: String = generate_file_name(name_conf)?;
	let mut generated_file: File = OpenOptions::new().write(true).create_new(true).open(dir_path.join(file_name))?;
	let file_content: Vec<u8> = generate_file_content(content_conf)?;
	generated_file.write_all(& file_content)?; // TODO Do buffered write
	Ok(())
    } else {
	Err(Box::new(WrongConfigDataError)) // TODO raise another error
    }
}

pub fn generate_directory(conf: & ItemConfig, dir_path: & Path) -> Result<(), Box<dyn Error>> {
    if let ItemConfig::DirectoryConfig {name: name_conf, items: items_conf} = conf {
	// TODO
	Ok(())
    } else {
	Err(Box::new(WrongConfigDataError)) // TODO raise another error
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_generate_bit() {
	unimplemented!()
    }

    #[test]
    fn test_generate_byte_1() {
	unimplemented!()
    }

    #[test]
    fn test_generate_byte_2() {
	unimplemented!()
    }
    
    #[test]
    fn test_generate_file() {
	unimplemented!()
    }

    #[test]
    fn test_generate_directory() {
	unimplemented!()
    }
}
