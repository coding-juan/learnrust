pub fn run() {
  println!("Macros!");
}

// Declarative Macros
macro_rules! myvec {
  ( $( $x:expr ),* ) => {
    let mut temp_vec = Vec::new();
    $(
      temp_vec.push($x);
    )
    temp_vec
  }
}