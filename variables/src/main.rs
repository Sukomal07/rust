//we can declare const in global scope. but can not declare let in global scope.

const GLOBAL: i32 = 24;

fn main() {
    //by default variables are immutable.
    // to make mutable variable add mut keyword.
    let mut age = 24;

    println!("value of {age}");

    age = 32;

    println!("value of {age}");

    //for const we can not use mut and we need to give types

    const PI: f64 = 3.14;
    println!("PI value is {PI}");
    println!("global scope {GLOBAL}");

    //this is callled shadowing
    // we can re declare same variable and variable value will be previous value

    let x = 5;

    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
}
