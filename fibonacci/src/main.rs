use std::io;
use std::collections::HashMap;

fn fibonacci(num: u64, cache: &mut HashMap<u64, u64>) -> u64 {
  if let Some(value) = cache.get(&num) {
    return *value
  }

  match num {
    0 => {
      cache.insert(num, 0);
      return 0
    },
    1 => {
      cache.insert(num, 1);
      return 1
    },
    _ => {
      let value = fibonacci(num - 1, cache) + fibonacci(num - 2, cache);
      cache.insert(num, value);
      return value
    }
  };
}

fn main() {
  let mut cache: HashMap<u64, u64> = HashMap::new();
  loop {
    println!("Enter n:");
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Failed to read line");
    let num: u64 = match num.trim().parse() {
      Ok(num) => num,
      Err(_) => {
        println!("Please enter a valid value");
        continue;
      }
    };
    println!("n: {}, result: {}", num, fibonacci(num, &mut cache));
  }
}
