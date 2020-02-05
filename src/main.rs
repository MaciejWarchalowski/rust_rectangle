struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width >= other.width && self.height >= other.height
    }

    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 10,
        height: 20
    };
    let rect2 = Rectangle {
        width: 10,
        height: 5
    };
    let rect3 = Rectangle::square(20);
    let res : Option<i32> = None;

    match res {
        Some(x) => println!("The number is {}", x),
        None => println!("Failed")
    }

    println!("Area is: {}", rect1.area());
    println!("Can hold? {}", rect1.can_hold(&rect2));
    println!("Area is: {}", rect3.area());
}
