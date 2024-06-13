mod read_input;
use capitalize::Capitalize;

use read_input::read_input;

fn main(){

    let ol_values = vec!["Respect", "Excellence", "Friendship"];

    let user = read_input("Name one olympic value: ").capitalize();

    if ol_values.contains(&user.as_str()) {
        println!("\nThat's correct!");
    } else {
        println!("Incorrect");
    }
    
    

}


