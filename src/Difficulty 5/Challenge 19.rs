mod read_input;
//use capitalize::Capitalize;

use read_input::read_input;

fn main(){

    loop {

        let user: i32 = read_input("Enter a number: ").parse().expect("Entere a number!!");

        if user == 7 {
            println!("Well Done!");
            break;
        }

    }
    

}



