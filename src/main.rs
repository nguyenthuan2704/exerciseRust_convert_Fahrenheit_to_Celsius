use std::io;

fn main() {
    println!("Program to convert temperature from Fahrenheit to Celsius");
    loop{
        println!("Enter the F temperature!");
        let mut f = String::new();
        io::stdin().read_line(&mut f).expect("please enter numbers instead of characters and letters!");
        let f: u32 = match f.trim().parse() {
            Ok(temp) => temp,
            Err(_) => {
                println!("please enter numbers instead of characters and letters!");
                continue;
            }
        };
        println!("The temperature F you entered:{f}");
        let c = {
            (f - 32) * 5/9
        };
        println!("The temperature C is :{c}");
        break;
    }

}
