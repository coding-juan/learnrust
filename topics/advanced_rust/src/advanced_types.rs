pub fn run() {
  println!("Advanced Types!");

  println!("---- Type Synonyms/Type Aliases");
  type Kilometers = i32;
  let x: i32 = 5;
  let y: Kilometers = 5;
  println!("------ x + y = {}", x + y);

  type Thunk = Box<dyn Fn() + Send + 'static>;
  let f: Thunk = Box::new(|| println!("hi"));
  fn takes_thunk(f: Thunk) {}
  fn returns_think() -> Thunk {
    Box::new(|| ())
  }
}