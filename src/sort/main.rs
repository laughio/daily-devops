use std::{env, io};
use std::io::BufRead;

fn main() {
  let argv: Vec<String> = env::args().collect();
  let sep = if argv.len() >= 2 { &argv[1] } else { " " };
  loop {
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut handle = stdin.lock();
    let r = handle.read_line(&mut buffer);
    match r {
      Ok(l) => if l > 0 {
        let mut pisces: Vec<String> = vec![];
        let parts = buffer.split(sep);
        parts.for_each(|p| { pisces.push(p.parse().unwrap()); });
        pisces.sort();
        println!("{}", pisces.join(&sep));
      }
      _ => {}
    }
  }
}