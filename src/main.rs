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
                userchoice_handle1();
                break;
            } else if userchoice == 2 {
                userchoice_handle2();
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

fn userchoice_handle1() {
    let mut userchoice_convertftoc = String::new();

    println!("Please enter a value to convert from fahrenheit to celsius.");

    io::stdin()
        .read_line(&mut userchoice_convertftoc)
        .expect("Failed to read line");

    let userchoice_convertftoc: f64 = match userchoice_convertftoc.trim().parse() {
        Ok(num) => {
            let rvalue: f64 = convertftoc(num);
            rvalue
        }
        Err(_) => 0.0,
    };

    let userchoice_convertftoc_string = userchoice_convertftoc.to_string();

    println!("The result is: {userchoice_convertftoc_string}");
}

fn userchoice_handle2() {
    let mut userchoice_convertftoc = String::new();

    println!("Please enter a value to convert from fahrenheit to celsius.");

    io::stdin()
        .read_line(&mut userchoice_convertftoc)
        .expect("Failed to read line");

    let userchoice_convertftoc: f64 = match userchoice_convertftoc.trim().parse() {
        Ok(num) => {
            let rvalue: f64 = convertctof(num);
            rvalue
        }
        Err(_) => 0.0,
    };

    let userchoice_convertftoc_string = userchoice_convertftoc.to_string();

    println!("The result is: {userchoice_convertftoc_string}");
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
