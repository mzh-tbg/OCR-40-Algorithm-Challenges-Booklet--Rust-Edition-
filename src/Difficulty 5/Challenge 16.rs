
mod read_input;
//use capitalize::Capitalize;

use read_input::read_input;

fn main(){

    let user: u32 = read_input("How long do you spend watching TV each day: (Nearest Hour) ").parse().expect("Please enter a number between 0-24h");

    if user < 2 {
        println!("That should be ok");
    } else if user <=4 {
        println!("That will rot your brain!");
    } else {
        println!("That is too much TV!");
    }

    
    

}


