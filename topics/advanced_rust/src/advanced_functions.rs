pub fn run() {
  println!("Advanced Functions & Closures");
  println!("---- Function Pointers");
  let answer = do_twice(add_one, 5);
  println!("------ Result: {}", answer);

  println!("---- Returning Closures");
  let closure = returns_closure();
  let value = Some(1);
  let result = value.map(closure);
  println!("------ Value is: {}", result.unwrap());
}

// Function Pointers
fn add_one(x: i32) -> i32 {
  x + 1
}

fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
  f(arg) + f(arg)
}

// Returning Closures
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
  Box::new(|x| x + 1)
}