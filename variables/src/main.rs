use std::io;

fn main() {
    loop {
        println!("Convert to Celcius: ");
        let mut fah = String::new();
        io::stdin().read_line(&mut fah).expect("enter something");
        let fah: i32 = match fah.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("enter a number");
                break
            },
        };

        let cel = (fah - 32) * 5/9;
        println!("A fahrenheit of {} is {} in celcius", fah, cel);
    }
}
