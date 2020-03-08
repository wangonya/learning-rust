#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let width = 30;
    let height = 50;
    println!("simple area = {}", simple_area(width, height));

    let rectupl = (30, 50);
    println!("tupl area = {}", tuple_area(rectupl));

    let rect_struct = Rectangle {width, height};
    println!("rect_struct = {:?}", rect_struct); // use {:#?} to beutify
    println!("struct area = {}", struct_area(&rect_struct));

    println!("rect method area = {}", rect_struct.mthd_area())
}

// the fn is supposed to calculate the area of 
// one rectangle, but the fn has 2 parameters
// the parameters are related but that isn't 
// expressed anywhere in the code
// not ideal
fn simple_area(width: u32, height: u32) -> u32 {
    width * height
}

// a little bit better but having to index
// not ideal
fn tuple_area(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

// use defined Rectangle struct
// ideal
fn struct_area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// even more ideal
// implement a method
impl Rectangle {
    fn mthd_area(&self) -> u32 {
        self.width * self.height
    }
}
