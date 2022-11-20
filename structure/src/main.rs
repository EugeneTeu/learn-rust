struct Rectangle {
    width: u32,
    height: u32,
}

enum ipv4 {
    v1,
    v2,
    v3,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let value = ipv4::v1;
    let a = Rectangle {
        width: 10,
        height: 20,
    };

    let b = Rectangle {
        width: 11,
        height: 25,
    };
    println!("can A hold B? {}", a.can_hold(&b));
    println!("can B hold A? {}", b.can_hold(&a));
}
