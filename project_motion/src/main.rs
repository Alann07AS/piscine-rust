use project_motion::*;

fn main() {
    let mut obj = ThrowObject::new(Object { x: -10.0, y: 50.0 }, Object { x: -10.0, y: -10.0 });
    println!("{:?}", obj.next());    println!("");
    println!("{:?}", obj.next());    println!("");
    println!("{:?}", obj.next());    println!("");
    println!("{:?}", obj.next());    println!("");
    println!("{:?}", obj.next());    println!("");
}


// ---- tests::test_with_negative_velocity stdout ----
// thread 'tests::test_with_negative_velocity' panicked at 'assertion failed: `(left == right)`
//   left: `Some(ThrowObject { init_position: Object { x: -10.0, y: 50.0 }, init_velocity: Object { x: -10.0, y: -10.0 }, actual_position: Object { x: -10.0, y: 50.0 }, actual_velocity: Object { x: -10.0, y: -10.0 }, time: 0.0 })`,
//  right: `Some(ThrowObject { init_position: Object { x: -10.0, y: 50.0 }, init_velocity: Object { x: -10.0, y: -10.0 }, actual_position: Object { x: -20.0, y: 35.1 }, actual_velocity: Object { x: -10.0, y: -19.8 }, time: 1.0 })`', src/main.rs:112:9
// note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

// ---- tests::test_with_velocity stdout ----
// thread 'tests::test_with_velocity' panicked at 'assertion failed: `(left == right)`
//   left: `Some(ThrowObject { init_position: Object { x: 0.0, y: 50.0 }, init_velocity: Object { x: 10.0, y: 10.0 }, actual_position: Object { x: 0.0, y: 50.0 }, actual_velocity: Object { x: 10.0, y: 10.0 }, time: 0.0 })`,
//  right: `Some(ThrowObject { init_position: Object { x: 0.0, y: 50.0 }, init_velocity: Object { x: 10.0, y: 10.0 }, actual_position: Object { x: 10.0, y: 55.1 }, actual_velocity: Object { x: 10.0, y: 0.2 }, time: 1.0 })`', src/main.rs:61:9

// ---- tests::test_without_acelaration_velocity stdout ----
// thread 'tests::test_without_acelaration_velocity' panicked at 'assertion failed: `(left == right)`
//   left: `Some(ThrowObject { init_position: Object { x: 50.0, y: 50.0 }, init_velocity: Object { x: 0.0, y: 0.0 }, actual_position: Object { x: 50.0, y: 50.0 }, actual_velocity: Object { x: 0.0, y: 0.0 }, time: 0.0 })`,
//  right: `Some(ThrowObject { init_position: Object { x: 50.0, y: 50.0 }, init_velocity: Object { x: 0.0, y: 0.0 }, actual_position: Object { x: 50.0, y: 45.1 }, actual_velocity: Object { x: 0.0, y: -9.8 }, time: 1.0 })`', src/main.rs:19:9

