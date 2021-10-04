use serde::{Deserialize};

#[derive(Deserialize)]
pub enum RngConfig {
    CycleGen {
	value: Vec<u8>,
	length: usize,
    },
    RangeGen {
	value: Vec<Vec<u8>>,
	seed: Option<Vec<u8>>,
	length: usize,
    },
    BitTrain {
	seed: Option<Vec<u8>>,
	length: usize,
    },
    StringGen {
	value: String,
    }
}

#[derive(Deserialize)]
pub enum ItemConfig {
    FileConfig {
	name: RngConfig,
	rand_gen: RngConfig,
    },
    DirectoryConfig {
	name: RngConfig,
	items: Vec<ItemConfig>,
    },
}

pub fn validate_config () {
    unimplemented!();
}

#[cfg(test)]
mod test_config {
    use super::ItemConfig;
    use super::RngConfig;
    use std::error::Error;
    use crate::errors::WrongConfigDataError;
    
    #[test]
    fn test_parse_config() -> Result<(), Box<dyn Error>> {
	let test: ItemConfig = serde_json::from_str("\"FileConfig\":{\"name\":{\"CycleGen\":{\"value\":[116,101,115,116,95,102,105,108,101,46,116,120,116],\"length\":13}},\"rand_gen\":{\"BitTrain\":{\"seed\":[1,2,3,4,5,6],\"length\":27}}}}")?;
	if let ItemConfig::FileConfig{
	    name: _,
	    rand_gen: t
	} = test {
	    if let RngConfig::BitTrain{
		seed: tt,
		length: _
	    } = t {
		assert_eq!(tt, vec![1, 2, 3, 4, 5, 6]);
		Ok(())
	    } else {
		Err(Box::new(WrongConfigDataError))
	    }
	} else {
	    Err(Box::new(WrongConfigDataError))
	}
    }
}
