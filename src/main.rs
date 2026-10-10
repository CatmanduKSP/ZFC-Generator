use std::io::{self, Write};

fn main(){
    let mut raw_brackets = String::new();
    let mut final_zfc = String::new();

    let nat_number: u8 = loop {
        println!("Enter natural number...");
        let mut input_text = String::new();
        io::stdin()
            .read_line(&mut input_text)
            .expect("FATAL ERROR");
        match input_text.trim().parse() {
            Ok(number) => {
                if number < 127 {
                    break number;
                } else {
                    println!("Too large.");
                }
            },
            Err(_) => println!("Invalid entry."),
        }
    };
 
    println!("Output length: {}", (2_u128 * (2_u128.pow(nat_number as u32))));
    for i in 0..=nat_number {

        raw_brackets.push_str(&final_zfc);
        final_zfc = format!("{{{}}}", raw_brackets);

        if nat_number == 0 {print!("\r100%...")} else {print!("\r{}%...", (i as u32 * 100) / nat_number as u32)};
        io::stdout().flush().unwrap();
    }

    println!("\n{}", final_zfc);

}
