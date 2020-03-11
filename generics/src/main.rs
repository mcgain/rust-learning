struct Point<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

pub trait Drawable {
    fn draw(&self) -> Result<String, String>{
        Result::Ok(format!(
            "Drew Point<{}, {}, {}>", self.x, self.y, self.z),
        )
    }
}

impl Drawable for Point<i32>{}

fn main() {
    println!("Generics");
    let a = Point { x: 0, y: 0, z: 0 };
    let drawing = a.draw().unwrap();
    println!("{}", drawing);
}
