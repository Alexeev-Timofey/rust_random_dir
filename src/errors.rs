use std::fmt::{Display, Debug, Formatter};
use std::error::Error;

pub struct WrongConfigDataError;
impl Display for WrongConfigDataError {
    fn fmt(& self, f: & mut Formatter<'_>) -> std::fmt::Result {
	write!(f, "Wrong config data")
    }
}
impl Debug for WrongConfigDataError {
    fn fmt(& self, f: & mut Formatter<'_>) -> std::fmt::Result {
	write!(f, "Wrong config data")
    }
}
impl Error for WrongConfigDataError {}
