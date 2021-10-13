mod config;
mod errors;
mod extern_api;

use std::error::Error;
use std::iter::Iterator;
use std::path::{Path, PathBuf};
use std::fs::{File, OpenOptions, create_dir};
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

fn generate_bit(acc: GenByteResult, bit_change: (usize, bool)) -> GenByteResult {
    let val_bit: bool = bit_change.1 ^ acc.bit;
    let val: u8 = if val_bit {
	acc.byte + (1 << (7 - bit_change.0))
    } else {
	acc.byte
    };
    GenByteResult {byte: val, bit: val_bit}
}

pub fn generate_byte(distr: & Bernoulli, rng: & mut impl Rng, init_bit: bool) -> GenByteResult {
    let init_val = GenByteResult {byte: 0u8, bit: init_bit};
    rng.sample_iter(distr).enumerate().take(8).fold(init_val, generate_bit)
}

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

pub fn generate_file(conf: & ItemConfig, dir_path: & Path) -> Result<(), Box<dyn Error>> {
    match conf {
	ItemConfig::FileConfig {
	    name: name_conf,
	    rand_gen: content_conf
	} => {
	    let file_name: String = generate_file_name(name_conf)?;
	    let mut generated_file: File = OpenOptions::new().write(true).create_new(true).open(dir_path.join(file_name))?;
	    let file_content: Vec<u8> = generate_file_content(content_conf)?;
	    generated_file.write_all(& file_content)?; // TODO Do buffered write
	    Ok(())
	},
	ItemConfig::DirectoryConfig {
	    name: name_conf,
	    items: items_conf
	} => {
	    let file_name: String = generate_file_name(name_conf)?;
	    let new_dir_path: PathBuf = dir_path.join(file_name);
	    create_dir(& new_dir_path)?;
	    for item in items_conf { // TODO paralell this (iterators?)
		generate_file(item, & new_dir_path)?;
	    }
	    Ok(())
	}
    }
}

// TODO Move tests to separete directory
#[cfg(test)]
mod tests {
    use rand::prelude::*;
    use rand::distributions::Bernoulli;
    use rand_xorshift::XorShiftRng;
    use std::error::Error;
    use std::fmt::{Display, Debug, Formatter};
    use std::ops::Not;
    use crate::config::*;
    use crate::{generate_file_name, generate_bit, generate_byte, GenByteResult};

    struct TestError {
	message: String
    }

    impl TestError {
	fn new(msg: String) -> TestError { // FIXME message type
	    TestError{message: msg}
	}
    }

    impl Display for TestError {
	fn fmt(& self, f: & mut Formatter<'_>) -> std::fmt::Result {
	    write!(f, "{}", self.message)
	}
    }

    impl Debug for TestError {
	fn fmt(& self, f: & mut Formatter<'_>) -> std::fmt::Result {
	    write!(f, "{}", self.message)
	}
    }

    impl Error for TestError {}
    
    #[test]
    fn test_generate_file_name_bittrain() -> Result<(), Box<dyn Error>> {
	let conf: RngConfig = RngConfig::BitTrain {
	    probability: 0.5,
	    seed: None,
	    length: 12
	};
	if generate_file_name(& conf).is_err() {
	    Ok(())
	} else {
	    Err(Box::new(TestError::new("Created file name with bittrein generator".to_string())))
	}
    }

    #[test]
    fn test_generate_file_name_cycle() -> Result<(), Box<dyn Error>> {
	let test_val: String = "teeeeeest".to_string();
	let conf: RngConfig = RngConfig::CycleGen {
	    value: test_val.clone().into_bytes(),
	    length: test_val.len()
	};
	let test_test: String = generate_file_name(& conf)?;
	if test_val == test_test {
	    Ok(())
	} else {
	    Err(Box::new(TestError::new("Test values not equal".to_string())))
	}
    }

    #[test]
    fn test_generate_file_name_range() -> Result<(), Box<dyn Error>> {
	let conf: RngConfig = RngConfig::RangeGen {
	    value: vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
	    seed: None,
	    length: 12
	};
	if generate_file_name(& conf).is_err() {
	    Ok(())
	} else {
	    Err(Box::new(TestError::new("Created file name with range generator".to_string())))
	}
    }

    #[test]
    fn test_generate_file_name_string() -> Result<(), Box<dyn Error>> {
	let test_val: String = "teeeeeest".to_string();
	let conf: RngConfig = RngConfig::StringGen {
	    value: test_val.clone()
	};
	let test_test: String = generate_file_name(& conf)?;
	if test_val == test_test {
	    Ok(())
	} else {
	    Err(Box::new(TestError::new("Test values not equal".to_string())))
	}
    }
    
    #[test]
    fn test_generate_bit() -> Result<(), Box<dyn Error>> {
	let test_byte: u8 = (1 << 7) + (1 << 6) + (1 << 5);
	let test_bit: bool = false;
	let first_byte_shift: usize = 4;
	let first_bit_change: bool = false;
	let second_byte_shift: usize = 3;
	let second_bit_change: bool = true;
	let init_val: GenByteResult = GenByteResult {
	    byte: test_byte,
	    bit: test_bit
	};
	let intermediate_val: GenByteResult = generate_bit(init_val, (first_byte_shift, first_bit_change));
    	assert_eq!(intermediate_val.byte, test_byte);
	assert_eq!(intermediate_val.bit, test_bit);
	let final_val: GenByteResult = generate_bit(intermediate_val, (second_byte_shift, second_bit_change));
	assert_eq!(final_val.byte, (test_byte + (1 << (7 - second_byte_shift))));
	assert_eq!(final_val.bit, test_bit.not());
	Ok(())
    }

    #[test]
    fn test_generate_byte_1() -> Result<(), Box<dyn Error>> {
	let distrib: Bernoulli = Bernoulli::new(1.0)?;
	let rand_seed: [u8; 16] = [1; 16];
	let mut rand_gen: XorShiftRng = XorShiftRng::from_seed(rand_seed);
	let result = generate_byte(& distrib, & mut rand_gen, false);
	assert_eq!(result.byte, 170);
	assert!(result.bit.not());
	Ok(())
    }

    #[test]
    fn test_generate_byte_0() -> Result<(), Box<dyn Error>> {
	let distrib: Bernoulli = Bernoulli::new(0.0)?;
	let rand_seed: [u8; 16] = [1; 16];
	let mut rand_gen: XorShiftRng = XorShiftRng::from_seed(rand_seed);
	let result = generate_byte(& distrib, & mut rand_gen, true);
	assert_eq!(result.byte, 255);
	assert!(result.bit);
	Ok(())
    }
}
