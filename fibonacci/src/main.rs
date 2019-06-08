use std::io;

fn fibonacci(num: u32) -> u32 {
  match num {
    0 => 0,
    1 => 1,
    _ => fibonacci(num - 1) + fibonacci(num - 2)
  }
}

fn main() {
  loop {
    println!("Enter n:");
    let mut num = String::new();
    io::stdin().read_line(&mut num).expect("Failed to read line");
    let num: u32 = match num.trim().parse() {
      Ok(num) => num,
      Err(_) => {
        println!("Please enter a valid value");
        continue;
      }
    };
    println!("n: {}, result: {}", num, fibonacci(num));
  }
}
