use std::io;

fn main() {
    loop {
        let mut input = String::new();
        println!("Please Enter a geek word (or 'exit' to quit):");

        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "exit" {
            break;
        }
    }
}
