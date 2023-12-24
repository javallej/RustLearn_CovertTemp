use std::io;


fn main() {
    userhandle();
}


fn userhandle() {

    'main_control_loop: loop {

    let mut userchoice = String::new();


    loop {
        println!("Please enter 1 to convert from fahrenheit to celsius.");
        println!("Please enter 2 to convert from celsius to fahrenheit.");
        println!("Please enter 3 to quit.");

        io::stdin()
        .read_line(&mut userchoice)
        .expect("Failed to read line");

        let userchoice: u32 = match userchoice.trim().parse() {
            Ok(num) => num,
            Err(_) => break,
        };


        if userchoice == 1 {
            let mut user_valuef = convertftoc(40.0);
            println!("The value is: {user_valuef}");
            break;
        } else if userchoice == 2 {
            let mut user_valuec = convertctof(32.0);
            println!("The value is: {user_valuec}");
            break;
        } else if userchoice == 3 {
            println!("Bye!");
            break 'main_control_loop;
        } else {
            println!("Please type a number (1 or 2 or 3)!");
            break 'main_control_loop;
        }
    }

    }
}

fn convertftoc(f: f64) -> f64 {
println!("Convert from fahrenheit to celsius.");

let cvalue = f - 32.0 / 1.8000;

cvalue

}

fn convertctof(c: f64) -> f64 {
println!("Convert from celsius to fahrenheit.");

let fvalue = c * 1.8000 + 32.00;

fvalue

}