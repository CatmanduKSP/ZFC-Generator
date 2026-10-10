use std::io;

fn main(){
    let mut raw_brackets = String::new();
    let mut final_zfc = String::new();
    let nat_number: u128 = loop {
        println!("Enter natural number...");
        let mut input_text = String::new();
        io::stdin()
            .read_line(&mut input_text)
            .expect("FATAL ERROR");
        match input_text.trim().parse() {
            Ok(number) => break number,
            Err(_) => println!("Invalid entry.\n"),
        }
    };
    
    for i in 0..=nat_number {

        raw_brackets.push_str(final_zfc);
        let final_zfc = format!("{{{}}}", raw_brackets);

        print!("\r{}%...", (i * 100) / nat_number);
        io::stdout().flush().unwrap();
    }

    println!(final_zfc);
}
