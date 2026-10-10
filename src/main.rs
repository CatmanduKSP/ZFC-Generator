use std::io;

fn main(){
    let nat_number: u128 = loop {
        println!("Enter natural number...");
        let mut input_text - String::new();
        io::stdin()
            .read_line(&mut input_text)
            .expect("FATAL ERROR");
        match input_text.trim().parse() {
            Ok(number) => break number,
            Err(_) => println!("Invalid entry.\n"),
        }
    };

}
