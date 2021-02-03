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
    
    match lines.iter().find(|x| x.len() % 3 != 0){
        Some(x) => Err(Error::InvalidColumnCount(x.len())),
        None if lines.len() % 4 != 0 => Err(Error::InvalidRowCount(lines.len())),
        _ => get_ocr_number(lines)
    }
}

// Separate the digits in a vector and convert them into a string of numbers
pub fn get_ocr_number(lines: Vec<&str>) -> Result<String, Error>{
    // Hashmap with flattened version of the numbers
    let mut map: HashMap<&str,&str> = HashMap::new();
    map.insert("     |  |   ","1");
    map.insert(" _  _||_    ","2"); 
    map.insert(" _  _| _|   ","3"); 
    map.insert("   |_|  |   ","4"); 
    map.insert(" _ |_  _|   ","5"); 
    map.insert(" _ |_ |_|   ","6"); 
    map.insert(" _   |  |   ","7"); 
    map.insert(" _ |_||_|   ","8"); 
    map.insert(" _ |_| _|   ","9"); 
    map.insert(" _ | ||_|   ","0"); 

    let vec = extract_numbers(lines);
    let mut numbers = String::new();
    // let mut iter = vec.iter();
    // Why does while let on vec.iter().next() lead to an infinite loop?
    // while let Some(digit) = iter.next(){
    for digit in vec{
        numbers += get_number(&map, &digit);
    };

    Ok(numbers)
}

// Turn a vector with many digits into a vector of vectors that hold single digits
pub fn extract_numbers(lines: Vec<&str>) -> Vec<String>{
    // How many digits are there
    let horizontal_qty = lines[0].len() / 3; 
    let vertical_qty = lines.len() / 4;  
    let quantity = horizontal_qty * vertical_qty + (vertical_qty - 1);
    let mut ratchet = 0;
    // let mut skip = 4;

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
            // println!("{}.{}", i, pos+ratchet);
            digits[pos+ratchet].push_str(String::from_iter(row.iter()).as_str());
        });

        // Join digits from different lines from the first line (step 4)
        // E.g.: the 1st digit of 2nd row becomes 4th digit of the only row
        if (i+1) % 4 == 0 { 
            ratchet += horizontal_qty;
            if ratchet < quantity {
                digits[ratchet].push_str(",");
                ratchet += 1;
            }
        };
    };

    digits
}

// Convert a single digit into a number (returned as &str)
pub fn get_number<'a>(map: &HashMap<&str,&'a str>, digit: &'a str) -> &'a str {
    match digit{
        "," => digit,
        _ => map.get(digit).unwrap_or(&"?")
    }
}




