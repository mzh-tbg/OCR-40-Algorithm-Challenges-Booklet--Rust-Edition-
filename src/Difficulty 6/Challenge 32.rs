mod read_input;
//use capitalize::Capitalize;
use read_input::read_input;
use std::collections::HashMap;

fn main() {

    let mut dicts = HashMap::new();
    

    let money: f64 = read_input("How much money do you want to save: ")
        .parse()
        .expect("Please enter a sesnsible float");

    let banks: u32 = read_input("How many bank accounts do you want to compare: ")
        .parse()
        .expect("Please enter a sensible positivte integer");

    for x in 1..=banks {
        let rate: f64 = read_input(format!("What is the intrest rate for bank account {}:  ", x).as_str())
            .parse()
            .expect("Please enter a sensible float");

        let intrest = money / 100.0  * rate;

        dicts.insert(format!("Bank {}", x), intrest+money);

    }
    
    let mut total = 0.00;

    for (key, value) in dicts {
        println!("{}: {}", key, value);
        total += value;
    }

    println!("Total: {}", total);
    



}



 


