use project_motion::*;

fn main() {
    let mut obj = ThrowObject::new(Object { x: 50.0, y: 50.0 }, Object { x: 10.0, y: 10.0 });
    println!("{:?}", obj.next());    println!("");
    println!("{:?}", obj.next());    println!("");
    println!("{:?}", obj.next());    println!("");
    println!("{:?}", obj.next());    println!("");
    println!("{:?}", obj.next());    println!("");
}