mod read_input;
//use capitalize::Capitalize;
use read_input::read_input;

fn main() {
    
    let mut lengths: Vec<f64>= Vec::new();

    for _x in 1..=3 {
        let user:f64 = read_input("Please input a length of the triangle:  ")
            .parse()
            .expect("Please enter a sensible number!");
        lengths.push(user)
    }

    if lengths[0] == lengths[1] || lengths[1] == lengths[2] || lengths[0] == lengths[2] {
        println!("This is iscocoles!");
    } else {
        println!("This is not iscocoles!");
    }



}



 


