use std::iter::FromIterator;
use std::collections::HashMap;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidRowCount(usize),
    InvalidColumnCount(usize),
}

// Catch format error or call conversion function
pub fn convert(input: &str) -> Result<String, Error> {
    let lines: Vec<&str> = input.lines().collect();
    
    match lines.iter().find(|x| x.len() != 3){
        Some(x) => Err(Error::InvalidColumnCount(x.len())),
        None if lines.len() % 4 != 0 => Err(Error::InvalidRowCount(lines.len())),
        _ => get_ocr_number(lines)
    }
}

// Separate the digits in a vector and convert them into a string of numbers
pub fn get_ocr_number(lines: Vec<&str>) -> Result<String, Error>{
    let vec = extract_numbers(lines);
    let mut numbers = String::new();
    let mut iter = vec.iter();
    // Why does while let on vec.iter().next() lead to an infinite loop?
    while let Some(digit) = iter.next(){
        println!("{}", digit);
        numbers += get_number(digit);
    };

    Ok(numbers)
}

// Turn a vector with many digits into a vector of vectors that hold single digits
pub fn extract_numbers(lines: Vec<&str>) -> Vec<String>{
    // How many digits are there
    let horizontal_qty = lines[0].len() / 3; 
    let vertical_qty = lines.len() / 4;  
    let quantity = horizontal_qty * vertical_qty;
    let mut ratchet = 0;

    // Create a vector for each digit
    let mut digits: Vec<String> = vec![String::new();quantity];
    // let mut digits: Vec<String> = vec![String::new();quantity]; //9

    for (i, line) in lines.iter().enumerate(){
        // Store each digit into a single row
        line.chars()
        .collect::<Vec<char>>()
        .chunks(3)
        .enumerate().for_each(|(pos,row)| {
            
            // println!("{}. {:?} / '{}'", pos, row, digits[pos+ratchet]);
            // println!("{}. {:?} / '{}'", pos, row, ratchet);
            digits[pos+ratchet].push_str(String::from_iter(row.iter()).as_str());

            
            //  Join digits from different lines from the first line (step 4)
            // E.g.: the 1st digit of 2nd row becomes 4th digit of the only row
            if i+1 % 5 == 0 { ratchet += horizontal_qty };

        });
    };

    digits
}

// Convert a single digit into a number (returned as &str)
pub fn get_number(digit: &str) -> &str {
    let mut digits: HashMap<&str,&str> = HashMap::new();
    digits.insert("     |  |   ","1");
    digits.insert(" _  _||_    ","2"); 
    digits.insert(" _  _| _|   ","3"); 
    digits.insert("   |_|  |   ","4"); 
    digits.insert(" _ |_  _|   ","5"); 
    digits.insert(" _ |_ |_|   ","6"); 
    digits.insert(" _   |  |   ","7"); 
    digits.insert(" _ |_||_|   ","8"); 
    digits.insert(" _ |_| _|   ","9"); 
    digits.insert(" _ | ||_|   ","0"); 

    digits.get(digit).unwrap_or(&"?")
}




