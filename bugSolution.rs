fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5];
    let index = 10;

    if let Some(num) = numbers.get(index) {
        println!("The value at index {} is: {}", index, num);
    } else {
        println!("Index {} is out of bounds", index);
        // Handle the out-of-bounds error appropriately.
        // For example, you could return an error, use a default value,
        // or log the error.
    }
} 