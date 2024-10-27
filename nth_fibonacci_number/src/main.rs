fn main() {
    let index = 7;
    let fibonacci_number = get_fibonacci_number(index);

    println!("The Fibonacci number of index {index} is {fibonacci_number}")
}

fn get_fibonacci_number(n: i32) -> i32 {
    let mut index = n;

    if index <= 1 {
        return index;
    }

    let mut previous_value = 0;
    let mut current_value = 1;

    while index > 1 {
        let sum = previous_value + current_value;

        previous_value = current_value;
        current_value = sum;

        index = index - 1;
    }

    return current_value;
}
