use std::fs;

// const a_str: &str = "aaa";

fn main() {
  let args: Vec<String> = std::env::args().collect();

  assert!(args.len() >= 2, "Not enough arguments provided. Please provide at least 1 arguments.");

  let url = &args[1];
  let output = "rust.md";
  
  println!("Fetching url: {}", url);
  let body = reqwest::blocking::get(url).unwrap().text().unwrap();

  println!("Converting html to markdown...");
  let md = html2md::parse_html(&body);

  fs::write(output, md.as_bytes()).unwrap();
  println!("Converted markdown has been saved in {}.", output);
}