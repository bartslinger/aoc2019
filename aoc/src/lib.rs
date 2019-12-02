use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufRead, Read};
use std::str::FromStr;

#[cfg(test)]
mod tests {
    #[test]
    fn it_fails() {
        assert_ne!(2 + 2, 4);
    }
}

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
    let mut v: Vec<T> = Vec::new();
    let mut file = File::open(filename)?;
    let mut line = String::new();
    file.read_to_string(&mut line)?;

    for item in line.split(',') {
        v.push(item.parse()?);
    }
    
    Ok(v)
}

pub fn value_from_file<T>(filename: &str) -> Result<T, Box<dyn Error>>
    where T: FromStr, T::Err: 'static + Error
{
    let mut file = File::open(filename)?;
    let mut line = String::new();
    file.read_to_string(&mut line)?;
    let val = line.parse()?;
    Ok(val)
}