extern crate derive_more;

use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

pub struct PhoneNumber(String);

use derive_more::{Display, FromStr};
#[derive(FromStr, Display)]
pub struct PhoneNumber2(String);

impl FromStr for PhoneNumber {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(PhoneNumber(s.to_string()))
    }
}

impl fmt::Display for PhoneNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Deref for PhoneNumber {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Years(u32);

impl From<u32> for Years {
    fn from(val: u32) -> Years {
        Years(val)
    }
}
fn print_strings(s: &str) {
    println!("I've been asked to print {}", s);
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // parse() uses FromStr
    let num: PhoneNumber = "Hello Newtype".parse()?;

    // Deref can be called when we take a reference. The function
    // takes a &str and our type can Deref from &PhoneNumber to &str.
    print_strings(&num);

    // you can also call from_str directly
    let num = PhoneNumber::from_str("555-1234")?;

    // Display gives you a to_string function
    let _num_as_string = num.to_string();

    // Display can also be called directly by println! or format!
    println!("Phone number is {}", num);

    // use derive_more::{Display, FromStr};
    let _num2: PhoneNumber2 = "111-400".parse()?;
    let num2 = PhoneNumber2::from_str("99999999")?;
    let _num2_as_string = num2.to_string();
    println!("Phone number is {}", num2);

    // We can call from directly
    let _years = Years::from(10);

    // By implementing `From<u32> for Years`, we also get
    // `Into<Years> for u32` for free!
    let years: Years = 10.into();

    println!("Years: {:?}", years);
    Ok(())
}
