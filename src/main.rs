use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let random_number: u8 = rng.gen_range(0..100);
    let mut input_string: String = String::new();
    println!("Guess a number between 0 and 100");

    
    let mut trimmed: &str;
    let mut guess :u8;
    let mut guessed: bool = false;

    println!("Enter your guess :");
    while guessed == false {
        input_string.clear();
        std::io::stdin().read_line(&mut input_string).unwrap();
        trimmed= input_string.trim();
        guess = validate_integer(trimmed);

        if guess == random_number {
            println!("You won!");
            guessed = true;
        } else if guess < random_number {
            println!("Number is bigger");
        } else if guess > random_number {
            println!("Number is lower");
        }
    }

}

fn validate_integer(str :&str) -> u8 {
    let mut input_string: String = String::new();
    let mut trimmed: &str;
    let mut parsed :Result<u8, std::num::ParseIntError> = str.parse::<u8>();
    let mut valid :bool = parsed.is_ok();
    //keep asking for input until user inputs an integer
    while valid == false {
        input_string.clear();
        println!("Not a number!");
        std::io::stdin().read_line(&mut input_string).unwrap();
        trimmed = input_string.trim();
        parsed = trimmed.parse::<u8>();
        valid = parsed.is_ok();
    }

    return parsed.unwrap();
}
