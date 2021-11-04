struct Point {
   x: f32,
   y: f32,
}
/*
   (x,y)
   P----------|
   |          |
   |          |
   |----------Q (x,y)


*/
struct Rectangle {
   top_left: Point,
   bottom_right: Point,
}
/*
   X = Px + Qx
   Y = Qy + Py
*/
fn rect_area(a: Rectangle) -> f32 {
  let x = a.top_left.x - a.bottom_right.x;
  let y = a.top_left.y - a.bottom_right.y;
  x.abs() * y.abs()
}

fn main() {
    let rec = Rectangle {
        top_left: Point{
            x: 0.0, y: 4.0
        },
        bottom_right: Point {
            x: 5.0, y: 0.0
        }
    };
    println!("the area is {}", rect_area(rec));
}
