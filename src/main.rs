struct Square {
    width: f32,
}

struct Rectangle {
    width: f32,
    height: f32,
}

trait Shape {
    fn area(&self) -> f32;
} 

impl Shape for  Rectangle {
    fn area(&self) -> f32 {
        self.width * self.height
    }
}


fn main() {

}
