pub mod intcode_computer;

use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{BufReader, BufRead};
use std::str::FromStr;


pub fn vector_from_file<T>(filename: &str) -> Result<Vec<T>, Box<dyn Error>>
    where T: FromStr, T::Err: 'static + Error
{
    let mut v: Vec<T> = Vec::new();
    let file = File::open(filename)?;
    let br = BufReader::new(file);

    for line in br.lines() {
        let num = line?.parse()?;
        v.push(num);
    }

    Ok(v)
}

pub fn vector_from_comma_separated_file<T>(filename: &str) -> Result<Vec<T>, Box<dyn Error>>
    where T: FromStr, T::Err: 'static + Error
{
    let line = fs::read_to_string(filename)?;
    let v = line.split(',')
        .map(|x| x.parse::<T>().unwrap())
        .collect();
    
    Ok(v)
}