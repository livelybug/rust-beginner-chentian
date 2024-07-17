use regex::Regex;
use std::str::FromStr;

pub trait Parse {
    fn parse(s: &str) -> Self;
  }
/* 
impl Parse for u8 {
  fn parse(s: &str) -> Self {
    let re: Regex = Regex::new(r"^[0-9]+").unwrap();
    if let Some(cap) = re.captures(s) {
      println!("{:?}", cap);
      println!("{:?}", &cap[0]);
      println!("{:?}", cap.get(0));

      cap.get(0).map_or(0, |s| {
        println!("{:?}", s.as_str());
        s.as_str().parse().unwrap_or(0)
      })
    } else {
      0
    }
  }
}
 */
impl<T> Parse for T
where
  T: FromStr + Default + std::fmt::Debug
{
  fn parse(s: &str) -> Self {
    let re: Regex = Regex::new(r"^[0-9]+(\.[0-9]+)?").unwrap();
    let d = || Default::default();
    if let Some(cap) = re.captures(s) {
      println!("{:?}", cap);
      println!("{:?}", &cap[0]);
      println!("{:?}", cap.get(0));

      cap.get(0).map_or(d(), |s| {
        println!("{:?}", s.as_str().parse::<T>().unwrap_or(d()) );
        s.as_str().parse().unwrap_or(d())
      })
    } else {
      d()
    }
  }
}


fn main() {
    println!("Hello, world!");
    println!("{:?}", u8::parse("123cbd"));
    println!("{:?}", u8::parse("123acbd"));
    println!("{:?}", u16::parse("525cbd"));
    println!("{:?}", f64::parse("525.34cbd"));    
}
