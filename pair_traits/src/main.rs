use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

struct Guitar {
    brand: String,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self {x: x, y: y}
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
    let p1: Pair<i32> = Pair { x: 1, y: 2};
    let p2: Pair<String> = Pair { x: String::from("a1"), y: String::from("a2") };
    let p3: Pair<bool> = Pair::new(true, false);
    p1.cmp_display();
    p2.cmp_display();
    p3.cmp_display();

    let p4: Pair<char> = Pair { x: 'a', y: 'b' };
    p4.cmp_display();

    let p5: Pair<Guitar> = Pair { x: Guitar { brand: String::from("Ibanez") }, y: Guitar { brand: String::from("Fender") }};

    p5.cmp_display();
}
