mod read_input;
//use capitalize::Capitalize;

use read_input::read_input;

fn main(){

    let hours:i64 = read_input("How many hours have you worked: ").parse().expect("Please enter a sensible number!");
    let teady:i64 = read_input("How many teadys have you made!: ").parse().expect("Please enter a sensible number!");
    

    if (hours*5) > (teady*2) {
        println!("More money because of the hours: ${}", hours*5)
    } else if (hours*5) < (teady*2) {
        println!("More money because of the teadys: ${}", 2*teady)
    } else {
        println!("Hours and teadys pay is equal: ${}", 2*teady)
    }
    
    

}



