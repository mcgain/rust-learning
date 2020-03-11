#[derive(Debug)]
struct Rectangle {
    height: u32,
    width: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            height: size,
            width: size,
        }
    }
}

fn main() {
    let r1 = Rectangle {
        height: 13,
        width: 34,
    };

    let _manual_area = calculate_area(&r1);

    println!("area of rect {:?} is {}", r1, r1.area());

    println!("area of square size 34: {}", Rectangle::square(34).area());
}

fn calculate_area(r: &Rectangle) -> u32 {
    r.height * r.width
}
