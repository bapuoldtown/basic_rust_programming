fn main() {
    println!("Hello, world!");
    //Borrowwing concepts hands on examples
    let s1= String::from("Guru");
    let s2 = s1;
    //println!("The string s1 is {}", s1);//s1 value has been moved to s2 so there is no point calling s1 it will result in compile error
    println!("The new value has been moved to s2 {}", s2);
    //Move vs Copy Semantics: Deep Dive
    let a: i32 = 10;
    let b = a;
    //loop to brint two values :)
    //Copy means bitwise duplication. Rust literally copies every byte from one memory location to another. After the copy, you have two completely independent values.
    let mut counter = 2;
    while counter !=0{
        println!("{}",a);
        counter -= 1;
        println!("{}", b);
        counter -=1;
    };
    //Copy means bitwise duplication. Rust literally copies every byte from one memory location to another. After the copy, you have two completely independent values.
    //YES → MOVE (String, Vec, Box, File, custom structs with these)
    //NO → COPY (integers, floats, bool, char, references, tuples/arrays of Copy types)

    let int: i64 = 100;
    let int_copy: i64 = int;
    println!("The value of int is: {}, and the value of int_copy is: {}", int, int_copy);

    let flt: f64=10.325;
    let flt_copy: f64=flt;
    println!("The value of flt is: {}, and the value of flt_copy is: {}", flt, flt_copy);

    let flag: bool = true;
    let flag_copy: bool = flag;
    println!("The value of flag is: {}, and the value of flag_copy is: {}", flag, flag_copy);

    // Char - 4 bytes (Unicode scalar)
    let ch: char = '🦀';
    let ch_copy = ch;
    println!("{} {}", ch, ch_copy);  // Both valid

    //Tuples of Copy types are also Copy
    let my_tup: (i32, f64, char) = (10, 20.325, 'g');
    let cpy_my_tup = my_tup;
    println!("The original tuple is {:?} and the copied tuple is {:?}", my_tup, cpy_my_tup);

    // Arrays of Copy types
    let my_arr: [i32;3] = [1,2,3];
    let cpy_my_arr = my_arr;
    println!("The original array is {:?} and the copied array is {:?}", my_arr, cpy_my_arr);

    let x: i32 = 10;
    let y: i32 = add_ten(x);
    println!("The value of x is: {}, and the value of y is: {}", x, y);
    //Move Semantics
    //Rusr has struct as its class based imeplemantation
    struct Point{
        x: i32,
        y: i32,
    }

    let p1 = Point{x:32, y:40};
    let p2 = p1;
    //println!("The value of p1 is: ({}, {})", p1.x, -->this will give error as this is move semantics here here storage is mainly in heap
    println!("x variable is {}, y variable is {}", p2.x, p2.y);
    //The default semantics of struct is move we can keep to customise to copy but the members of it should have copy semantics
    #[derive(Copy, Clone)]
    struct Dog{
        breed: char,
    }

    let d1 = Dog {breed: 'b'};
    let d2 = d1;
    println!("The breed of d1 is: {}, and the breed of d2 is: {}", d1.breed, d2.breed);
    //This shoulld go fine as we have supprssed the semantics from by default move to copy but please remember its member shoukld have copy semantics  
}

fn add_ten(x: i32) -> i32{
   
    return x+10;

}