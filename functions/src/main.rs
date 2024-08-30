fn top_function(x: i32, y: bool, z: &str) {
    println!("all values is {x}, {y} , {z}");
}

fn main() {
    println!("Hello, world!");
    my_function(); //function calling
    another_function(40);
    top_function(20, false, "sukomal");
    add(20, 23);
    add2(80, 70, 60);
}

fn my_function() {
    println!("another function");
}

fn another_function(x: i32) {
    println!("x value is {x}");
}

fn add(x: i32, y: i32) -> i32 {
    let res = x + y;
    return res;
}

fn add2(x: i32, y: i32, z: i32) -> i32 {
    // x + y + z; if we add ; it will be statement
    //we can return like this way
    x + y + z
}
