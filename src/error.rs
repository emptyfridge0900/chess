use std::fmt;
use std::error::Error;

#[derive(Debug)]
pub struct InvalidateInutError;

impl fmt::Display for InvalidateInutError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Test error")
    }
}
impl Error for InvalidateInutError{

}