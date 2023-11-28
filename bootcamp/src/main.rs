// Fizz buzz program
// 1. Create a project called bootcamp using Cargo
// 2. The main function should print a welcome message.
// 3. Write a 'fizz buzz' function that will be called from your main function.
// 1. The function should have a loop counting up to 301
// 2. If the count is divisible by 3, print "fizz"
// 3. If the count is divisible by 5 print "buzz"
// 4. If the count is divisible by 3 and 5 print "fizz buzz"
// 5. At the end print the number of times "fizz buzz" occurred.

fn fizz_buzz() {
    let mut counter = 0;

    for n in 1..=301 {

        // avoid repeated calculations
        let divisible_by_3 = n % 3 == 0;
        let divisible_by_5 = n % 5 == 0;

        if divisible_by_3 && divisible_by_5 {
            println!("fizz buzz");
            counter += 1;
        }
        else if divisible_by_3 {
            println!("fizz");
        } else if divisible_by_5 {
            println!("buzz");
        }
    }

    println!("The number of times 'fizz buzz' occurred: {}", counter);
}

fn main() {
    println!("Hello, this is fizz buzz!");

    fizz_buzz();
}
