pub mod ai_ml;
pub mod arrays_and_strings;
pub mod bubble_sort;
pub mod lib;
pub mod linear_algebra;

fn main() {
    let mut numbers = [64, 34, 25, 12, 22, 11, 90];
    println!("Before: {:?}", numbers);

    bubble_sort(&mut numbers);

    println!("After: {:?}", numbers);
}
