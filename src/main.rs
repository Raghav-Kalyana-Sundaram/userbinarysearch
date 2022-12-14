fn main() {
    println!("Hello, world!");
    let uppernumber = hello();
    findNumber(uppernumber);  
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
    let mut guessable_number = upperNumber / 2;
    println!("Is your number {}? (<=/>)", guessable_number);
    let mut result = String::new();
    std::io::stdin()
        .read_line(&mut result)
        .expect("Failed to read line");
    result = result.trim().to_string();
}
