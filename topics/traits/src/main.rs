mod generics;
mod traits;

use traits::{Describer, Shareable};

fn main() {
    let vector = vec![29, 39, 12, 499, 398];
    let largest = generics::largest(&vector);
    eprintln!("Largest is {}", largest);

    /////////////
    
    let point = generics::Point::new(2, 10);
    eprintln!("X is {} and Y is {}", point.x(), point.y());
    point.cmp_display();

    let point = generics::Point::new(2.0, 4.0);
    eprintln!("X is {} and Y is {}", point.x(), point.y());
    eprintln!("Dist from center is {}", point.dist_from_origin());

    /////////////
    
    let jon = traits::Jon::new();
    let lunar = traits::Lunar::new();

    jon.describe();
    describe(&lunar);

    share(&jon);
    eprintln!("{}", lunar.share());

    let some_string = "Hehehe im string".to_string();
    some_string.describe();
}

fn describe(item: &impl traits::Describer) {
    item.describe();
}

fn share(item: &impl traits::Shareable) {
    eprintln!("{}", item.share());
}