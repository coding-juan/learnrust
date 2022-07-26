use std::ops::Add;
use std::fmt;

pub fn run() {
  println!("Advanced Traits!");

  println!("---- Associated Types");
  let mut counter = Counter { count: 0 };
  println!("------ {:?}", counter.next().unwrap());
  for count in counter {
    println!("------ {:?}", count);
  }

  println!("---- Default Generic Type and Operator Overloading");
  let ml1 = Millimeters(10);
  let ml2 = Millimeters(20);
  let m1 = Meters(3);
  println!("------ {:?}", ml1 + ml2);
  println!("------ {:?}", ml1 + m1);

}

// Associated Types
struct Counter {
  count: u32,
}

impl Iterator for Counter {
  type Item = u32; // Associated type

  fn next(&mut self) -> Option<Self::Item> {
    if self.count < 5 {
      self.count += 1;
      Some(self.count)
    } else {
      None
    }
  }
}

// Default Generic Type and Operator Overloading
#[derive(Debug, Copy, Clone)]
struct Millimeters(u32);
struct Meters(u32);

impl Add for Millimeters { // Here we are using default generic which is Rhs=Self so Rhs=Millimeters
  type Output = Millimeters;
  fn add(self, other: Millimeters) -> Millimeters {
    Millimeters(self.0 + other.0)
  }
}

impl Add<Meters> for Millimeters {
  type Output = Millimeters;
  fn add(self, other: Meters) -> Millimeters {
    Millimeters(self.0 + (other.0 * 1000))
  }
}

// Supertraits
trait OutlinePrint: fmt::Display { // Super traits, requiring a trait to implement another trait
}

// Newtype pattern
struct Wrapper(Vec<String>);
// impl ftm::Display for Vec<String> {} // This will fail
impl fmt::Display for Wrapper {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "[{}]", self.0.join(","))
  }
}