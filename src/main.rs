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
            convertftoc();
            break;
        } else if userchoice == 2 {
            convertctof();
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

fn convertftoc() {
println!("Convert from fahrenheit to celsius.");

}

fn convertctof() {
println!("Convert from celsius to fahrenheit.");

}