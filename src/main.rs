use std::{
    fs::File,
    io::{self, BufRead, BufReader},
    path::Path,
};

fn main() {
    loop {
        let mut input = String::new();
        println!("Please Enter a geek word (or 'exit' to quit):");

        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input == "exit" {
            break;
        }

        let path = Path::new("dictionary.txt");
        let display = path.display();

        let file = match File::open(&path) {
            Err(_) => File::create(&path).unwrap(),
            Ok(file) => file,
        };

        let reader = BufReader::new(file);

        if reader.lines().any(|line| line.unwrap() == input) {
            println!("This word is already in the Geek dictionary!");
            continue;
        }
    }
}
