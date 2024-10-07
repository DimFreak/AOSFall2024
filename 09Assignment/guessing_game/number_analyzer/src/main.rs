fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    // Create an array of 10 integer numbers
    let numbers: [i32; 10] = [10, 15, 3, 7, 22, 30, 5, 2, 8, 13];

    // Iterate through the array using a for loop
    for number in numbers {
        // Check if the number is divisible by 3, 5, or both
        if number % 3 == 0 && number % 5 == 0 {
            println!("{}: FizzBuzz", number);
        } else if number % 3 == 0 {
            println!("{}: Fizz", number);
        } else if number % 5 == 0 {
            println!("{}: Buzz", number);
        } else {
            // Use the is_even function to determine if the number is even or odd
            if is_even(number) {
                println!("{}: Even", number);
            } else {
                println!("{}: Odd", number);
            }
        }
    }

    // Use a while loop to find and print the sum of all numbers
    let mut sum = 0;
    let mut index = 0;

    while index < numbers.len() {
        sum += numbers[index];
        index += 1;
    }
    println!("Sum of all numbers: {}", sum);

    // Use a loop to find and print the largest number in the array
    let mut largest = numbers[0];
    for number in numbers {
        if number > largest {
            largest = number;
        }
    }
    println!("Largest number is: {}", largest);
}
