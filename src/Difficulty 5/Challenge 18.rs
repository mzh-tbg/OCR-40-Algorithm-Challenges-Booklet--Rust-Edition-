mod read_input;
//use capitalize::Capitalize;

use read_input::read_input;

fn main(){

    
    (1..=20)
        .filter(|x| x % 2 != 0)
        .for_each(|x| println!("{}", x))
    

}