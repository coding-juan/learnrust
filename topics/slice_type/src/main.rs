fn main() {
    let mut s = String::from("hello world");
    let first = first_word(&s); // Immutable borrow here
    // s.clear(); // We cannot have a mutable borrow with an immutable one 
    println!("{}", first);
}

fn first_word(s: &str) -> &str { // Here we use &str (string literal ref) as a parameter so we can pass both &str and &String
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}