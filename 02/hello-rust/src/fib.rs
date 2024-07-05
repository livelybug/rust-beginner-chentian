fn fib_extract(mut a:u8, mut b:u8, mut i:u8) -> (u8, u8, u8) {
  let c = a + b;
  a = b;
  b = c;
  i += 1;

  (a, b, i)
}

fn fib_loop(n: u8) {
  let mut a = 1;
  let mut b = 1;
  let mut i = 2u8;
  
  loop {
      (a, b, i) = fib_extract(a, b, i);
      println!("next val is {}", b);
      
      if i >= n {
          break;
      }
  }
}

fn fib_while(n: u8) {
  let (mut a, mut b, mut i) = (1, 1, 2);
  
  while i < n {
    (a, b, i) = fib_extract(a, b, i);      
    println!("next val is {}", b);
  }
}

fn fib_for(n: u8) {
  let (mut a, mut b) = (1, 1);
  
  for _i in 2..n {
    (a, b, _) = fib_extract(a, b, 0);
    println!("next val is {}", b);
  }
}

pub fn call_fib() {
  let n = 10;
  fib_loop(n);
  fib_while(n);
  fib_for(n);
}