// errors3.rs
// This is a program that is trying to use a completed version of the
// `total_cost` function from the previous exercise. It's not working though!
// Why not? What should we do to fix it?
// Execute `rustlings hint errors3` for hints!



use std::num::ParseIntError;

#[derive(Debug)]
struct MyParseIntError;

impl std::fmt::Display for MyParseIntError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Failed to parse integer")
    }
}

impl std::error::Error for MyParseIntError {}

fn main() -> Result<(), MyParseIntError> {
    let mut tokens = 100;
    let pretend_user_input = "8";

    match total_cost(pretend_user_input) {
        Ok(cost) => {
            if cost > tokens {
                println!("You can't afford that many!");
                // Return a custom error when the cost is too high.
                Err(MyParseIntError)
            } else {
                tokens -= cost;
                println!("You now have {} tokens.", tokens);
                Ok(())
            }
        }
        Err(err) => {
            println!("Error parsing input: {:?}", err);
            // Return the original error as a custom error.
            Err(MyParseIntError)
        }
    }
}

pub fn total_cost(item_quantity: &str) -> Result<i32, MyParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>().map_err(|_| MyParseIntError)?;

    Ok(qty * cost_per_item + processing_fee)
}