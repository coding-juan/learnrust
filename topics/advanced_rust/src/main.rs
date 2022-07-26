mod unsafe_rust;
mod advanced_traits;
mod advanced_types;
mod advanced_functions;
mod macros;

fn main() {
    // Unsafe
    unsafe_rust::run();
    // Advanced traits
    advanced_traits::run();
    // Advanced types
    advanced_types::run();
    // Advanced functions
    advanced_functions::run();
    // Macros
    macros::run();
}
