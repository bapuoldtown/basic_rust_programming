fn main() {
    println!("Hello, world!");
    //globbal variables
    let a = 20;
    //local variables in rust are inside variables cop
    {
        let x = 10;
        let mut y = 20;
        println!("The local variables is {} and {}", x, y);
        println!("The global variables is {}", a);
    }
    println!("The new value of a is {}",a);
    //println!("The local variable is {y}", y); //should be error as we are accessing local variables in global scope
    //we cannot access local variables outside of thr scope

    //shadowing now what does this say the variable define in certain cope can share the same name as variables in global scope
    //dispaly below
    let outer_var = 200;
    {
        let outer_var = 100;
        println!("The outer variable is {} accessed from inner scopes", outer_var);
    }
    println!("The outer variables declared is {} from outer variables", outer_var);

    //Lets declare and define some simple variable in Rust
    fn test(){
        let mut x: i32 = 1000;
        let mut y = "Programming".to_string().clone();
        println!("x: {}",x);
        println!("y:{}", y);
        x=1100;
        println!("x:{}",x);
        println!("y:{}", y)

    }
    //calling the function withing main
    test();
    //Below is the list of scalar types:

    //Integers
    //Floats
    //Boolean
    //Character

    //Compound Type

    //They group multiple values in one variable. Below is the list of compound types:

    //Array
    //Tuple

    //Positional Arguments
    //Each value is assigned a number based on the order of occurrence. The first value is assigned 0, the next is assigned 1, and so on and so forth. The placeholder takes an integer positive number (greater than or equal to 0) which indicates the value to be inserted in the placeholder is to be picked from the list of values in a given order.
    fn pos_args(){
        println!("The argument sequence are {1}, {0}, {2}", "Hello", "Hello World","Good Morning");
    }
    //calling pos_args
    pos_args();
    //named arguments formatting
    fn named_args(){
        println!("The named arguments are {greet}, {name}", name="Guru", greet="Hello");
    }
    //invoke the functions
    named_args();
    //Basic Python eval functionality
    fn basic_eval(){
        println!("{}+{}={}",10,20,10+10);
    }
    //calling method
    basic_eval();
    //Debug traits in Rust
    fn debug_traits(){
        let inner1=100;
        let inner2=200;
        //we can put maultiple values inside a place holder with a single place holder
        println!("The values are {:?}", (inner1, inner2));
    }
    debug_traits()
}
