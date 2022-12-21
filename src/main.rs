fn main() {
    println!("Hello, world!");
    findNumber(100);
}

fn getNumber() -> i32 {
    let mut number = String::new();
    std::io::stdin().read_line(&mut number).expect("Failed to read line");
    number = number.trim().to_string();
    let number: i32 = number.parse().expect("Please type a number");
    number
}

fn hello() -> i32{
    println!("Welcome to the guessing game. There is a twist, I will try and guess your number. \nIf it's higher than my guess, type > and if it's lower, type <=.\n I will know if I found the number");
    std::thread::sleep(std::time::Duration::from_secs(3));
    println!("Let's begin!");
    println!("What's the upper value you want me to guess?");
    let userNumber = getNumber();
    return userNumber;
}


fn findNumber(upperNumber :i32) -> bool{
    let mut highervalue = upperNumber;
    let mut guessable_number = upperNumber / 2;
    let mut result = userInput(guessable_number);
    loop {
        if result == ">" {
          let holder_value = guessable_number;
          guessable_number = (highervalue-guessable_number)/2 + guessable_number;
          result = userInput(guessable_number);
        } else if result == "<" {
          highervalue = highervalue/2;
          guessable_number = highervalue - guessable_number/4;
          result = userInput(guessable_number);
        } else {
            println!(".");
            return true
        }
    }
}

fn userInput(number :i32) -> String{
    println!("Is your number {}? (<=/>)", number);
    let mut result = String::new();
    std::io::stdin()
        .read_line(&mut result)
        .expect("Failed to read line");
    result = result.trim().to_string();
    return result;
}
