mod config;
mod errors;

use std::error::Error;
use std::iter::Iterator;
use std::path::Path;

use crate::config::*;
use crate::errors::*;

pub fn generate_file_name(conf: ItemConfig) -> Result<String, Box<dyn Error>> {
    match conf {
	ItemConfig::FileConfig {
	    name: RngConfig::StringGen {value: f_name},
	    rand_gen: _
	} |
	ItemConfig::DirectoryConfig {
	    name: RngConfig::StringGen {value: f_name},
	    items: _
	} => Ok(f_name),
	ItemConfig::FileConfig {
	    name: RngConfig::CycleGen {
		value: name_vec,
		length: name_len
	    },
	    rand_gen: _
	} |
	ItemConfig::DirectoryConfig {
	    name: RngConfig::CycleGen {
		value: name_vec,
		length: name_len
	    },
	    items: _
	} => {
	    let name_iter: std::slice::Iter<u8> = (*name_vec).iter();
	    let new_name_vec: Vec<u8> = name_iter.cycle().take(name_len).copied().collect();
	    Ok(String::from_utf8(new_name_vec)?)
	},
	_ => Err(Box::new(WrongConfigDataError))
    }
}

pub fn generate_file_content(conf: ItemConfig) -> Result<Vec<u8>, Box<dyn Error>> {
    unimplemented!()
}

pub fn generate_file(conf: ItemConfig, dir_path: & Path) -> std::io::Result<()> {
    unimplemented!()
}

pub fn generate_directory(conf: ItemConfig, dir_path: & Path) -> std::io::Result<()> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_generate_file() {}

    #[test]
    fn test_generate_directory() {}
}
