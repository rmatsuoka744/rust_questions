fn main() {
    let operations = vec![
        |x: i32| -> Result<i32, &'static str> {
            if x > 0 {
                Ok(x + 1)
            } else {
                Err("Invalid input error.")
            }
        },
        |x: i32| -> Result<i32, &'static str> {
            if x % 2 == 0 {
                Ok(x * x)
            } else {
                Err("Invalid input error.")
            }
        },
        |x: i32| -> Result<i32, &'static str> {
            if x < 10 {
                Ok(x - 3)
            } else {
                Err("invalid input error.")
            }
        },
    ];

    let number: i32 = 1;

    let mut result = Ok(number);

    for op in operations.iter() {
        result = result.and_then(op);
        println!("{:?}", result);

        if let Err(e) = result {
            println!("Operation failed: {}", e);
            break;
        }
    }

    match result {
        Ok(value) => println!("Fainal result: {}", value),
        Err(_) => println!("Processing Failed."),
    }
}
