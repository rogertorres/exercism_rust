// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

pub fn convert(input: &str) -> Result<String, Error> {
    let lines: Vec<&str> = input.lines().collect();
    
    // Catch format error or call conversion function
    match lines.iter().find(|x| x.len() != 3){
        Some(x) => return Err(Error::InvalidColumnCount(x.len())),
        None if lines.len() != 4 => Err(Error::InvalidRowCount(lines.len())),
        _ => get_ocr_number(lines)
    }
}

pub fn get_ocr_number(lines: Vec<&str>) -> Result<String, Error>{
    todo!()
}