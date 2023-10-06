use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct InvalidaInutError{

}

impl fmt::Display for InvalidaInutError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Test error")
    }
}
impl Error for InvalidaInutError{

}