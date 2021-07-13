use std::collections::HashMap;
use std::io;

use crate::util::{Categories, handle_command};
use crate::util::CmdType::*;
use std::io::Write;

mod util;

fn main() {
    let mut employees: HashMap<Categories, Vec<String>> = HashMap::new();
    employees.insert(Categories::Sales, vec![String::from("Ramesh")]);
    employees.insert(Categories::Engineering, vec![String::from("Lee")]);
    employees.insert(Categories::HR, vec![String::from("Virat")]);

    println!(
        "\
    - - - - - - - - - - \n \
    Welcome to DJTech Command Admin\n \
    - - - - - - - - - - \n\n \
    Enter h or help for information regarding commands\n>"
    );
    let mut inp = String::new();
    loop {
        io::stdin().read_line(&mut inp).expect("Incorrect input!");
        inp = String::from(inp.trim());
        inp.to_lowercase();
        inp = inp.replace("\n", "");
        match (inp.split(" ").next().unwrap(), inp.split(" ").skip(1).next().is_none()) {
            ("add", false) | ("+", false) => handle_command(&mut employees, &inp, Add),
            ("remove", false) | ("-", false) => handle_command(&mut employees, &inp, Remove),
            ("get", false) | ("fetch", false) => handle_command(&mut employees, &inp, Get),
            ("help", true) | ("h", true) => println!("Commands:\nadd- adds a user\nremove - removes a user\nhelp - shows the help message\nquit - quits the program"),
            ("quit", true) | ("q", true) => break,
            _ => println!("Unrecognized input {}! Run help to see commands", inp)
        };
        print!("> ");
        let _ = io::stdout().flush();
        inp = String::new();
    }
}
