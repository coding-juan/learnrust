use super::generics::*;
use core::fmt::Display;
// Traits are SIMILAR to interfaces in other languages, with some differences

pub trait Shareable {
  fn share(&self) -> String;
}

pub trait Describer {
  fn describe(&self) {
    eprintln!("Describing: {}", self.inner_describe());
  }

  fn inner_describe(&self) -> String;
}

pub struct Jon {
}

impl Jon {
  pub fn new() -> Self {
    Self {
    }
  }
}

pub struct Lunar {
}

impl Lunar {
  pub fn new() -> Self {
    Self {
    }
  }
}

impl Shareable for Jon {
  fn share(&self) -> String {
    "Click to see more from Jon".to_string()
  }
}

impl Describer for Jon {
  fn inner_describe(&self) -> String {
    "JON".to_string()
  }
}

impl Shareable for Lunar {
  fn share(&self) -> String {
    "Click to see more from Lunar".to_string()
  }
}

impl Describer for Lunar {
  fn describe(&self) {
    eprintln!("I describe in another way: {}!", self.inner_describe());
  }

  fn inner_describe(&self) -> String {
    "Lunar Rocks!".to_string()
  }
}

////////////////////////////////////////////////
// Trait bounds to conditionally implemnt methods
impl<T: Display + PartialOrd> Point<T> {
  pub fn cmp_display(&self) {
    if self.x() >= self.y() {
      println!("The largest is X = {}", self.x());
    } else {
      println!("The largest is Y = {}", self.y());
    }
  }
}

// Blanket Implementations
impl<T: Display> Describer for T {
  fn inner_describe(&self) -> String {
    self.to_string()
  }
} 