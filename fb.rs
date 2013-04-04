#!/usr/local/bin/rust run
extern mod std;
fn main() {
  for int::range(1, 101) |i| {
    io::println(
      if i % 15 == 0 { ~"FizzBuzz" }
      else if i % 3 == 0 { ~"Fizz" }
      else if i % 5 == 0 { ~"Buzz" }
      else { int::to_str(i) }
    )
  }
}


