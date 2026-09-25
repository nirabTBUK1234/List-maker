use std::io::{self, Write, stdout};
use std::fs::OpenOptions;

fn main() {
    struct Row{
        item: String,
        symbol_no: String
    }

    struct ToDoList{
        rows: Vec<Row>,
        list_name: String
    }

    let mut row = Row{
        item: String::new(),
        symbol_no: String::new()
    };

    loop {
        print!("Enter your note(type 'exit' to quit):\n\t");

        io::stdout().flush().unwrap();
        
        io::stdin()
            .read_line(&mut row.item)
            .expect("Failed to read line");

        print!("Enter the symbol number:\n\t");

        io::stdout().flush().unwrap();
        
        io::stdin()
            .read_line(&mut row.symbol_no)
            .expect("Failed to read line");

        let mut file = OpenOptions::new()
            .create(true)  
            .append(true) 
            .open("notes.txt")
            .expect("could not open file");
        
            file.write_all(format!("{} ", row.symbol_no)
                .as_bytes())
                .expect("Failed to write to file");

            file.write_all(row.item
                .as_bytes())
                .expect("Failed to write to file");

            let condition :bool = row.item
                .trim()
                .to_ascii_lowercase() == "exit" || 
                row.item
                .trim()
                .to_ascii_lowercase() == "quit";


            if condition
            {
                break;
            }

            else 
            {
                continue;
            }
    }
}
