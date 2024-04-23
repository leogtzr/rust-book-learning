#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

#[derive(Debug)]
struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        return &self.y
    }
}

impl<f32> Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    // let wont_work = Point { x: 5, y: 4 };
    let p1 = Point { x: 1, y: 2 };
    let p2: Point<i32> = Point { x: 2, y: 3 };
    let p3 = Point2 { x: 2, y: 2.3 };

    println!("p1.x() = {}", p1.x());
}
