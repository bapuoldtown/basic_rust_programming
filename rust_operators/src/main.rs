fn main() {
    println!("Hello, world!");
    //In Rust we have binary and unary operators
    //Unary operators
    /*
        Borrow Expression
        Dereference Expression
        Negation Expression
        Logical Negation Expression
     */
    /*
        Arithmetic Expression
        Logical Expression
        Comparison Expression
        Bitwise Expressions
        Assignment Expression
        Compound Assignment Expression
        Typecast Expression
     */
    let a:i32 = 10;
    let b:i32=20;
    println!("The addition operator is {}", a+b);
    println!("The subtraction operator is {}", a-b);
    println!("The modulus operator is {}", b%a);
    println!("The division operator is {}", b/a);
    println!("The mu;tiplication operator is {}", a*b);
    println!("The compound operaor below");
    let mut c:i32 =10;
    c+=5;
    println!("c:{:?}",c);
    c*=5;
    println!("c:{:?}",c);
    
    c/=3;
    println!("c:{:?}",c);
    c%=4;
    println!("The value of c after compound operations is {}", c);
    //logical operaors
    let x:bool=true;
    let y:bool = false;
    println!("The AND operator is {}", x && y);
    println!("The OR operator is {}", x || y);
    println!("The NOT operator is {}", !x);
    //logical operaors
    let a:i32=10;
    let b:i32=20;
    println!("The equal to operator is {}", a==b);
    println!("The not equal to operator is {}", a!=b);
    println!("The greater than operator is {}", a>b);
    println!("The less than operator is {}", a<b);
    println!("The greater than equal to operator is {}", a>=b);
    println!("The less than equal to operator is {}", a<=b);

    //Bitwise operators
    let p:u8=5;
    let q:u8=5;
    println!("The AND bitwise operator is {}", p & q);
    println!("The OR bitwise operator is {}", p | q);
    println!("The XOR bitwise operator is {}", p ^ q);
    println!("The left shift operator is {}", p << 1);
    println!("The right shift operator is {}", p >> 1);
    //typecasting error
    let m:i32 =10;
    let n:f64 = (m as f64)/2.0;
    println!("The value of n is {}", (n as i8));
    //Integer and float can be type casted bidirectionally
    //Integer can be typecasted to String and viceversa
    let int_val:i32=100;
    let str_val = int_val.to_string();
    println!("The string value is {}", str_val);
    let new_str_val: &str = &str_val;
    let parsed_int_val:i32 = new_str_val.parse().unwrap();
    println!("The parsed integer value is {}", parsed_int_val);
    //String (&str) or character cannot be type casted to the data type of type integer or float.
    //Borrwing and dereferrencing Operators
    //shared borrowing operator we can read the referenced variable but cannot alter it
    let var1:i32 = 50;
    let ref_var1:&i32 = &var1;
    let ref_var2:&i32 = &ref_var1;
    println!("The value of ref_var1 is {}, {}", ref_var1, ref_var2);
    //mutable borrowing operator we can read and write the referrenced variables
    let mut var2:i32 = 100;
    let mut var3:i32 = 300;
    let mut var2_ref= &mut var2; //see here my refernce is also mutable now using this i can change the value its pointing and also change its own value also
    //let var2_ref2=&mut var2;
    *var2_ref +=10;
    var2_ref = &mut var3; //after changing the referred value i can now change the referrence to point to another mutable bindde value which is var3
    //let var2_ref2 =&var2_ref;
    //*var2_ref = 30;
    
    //println!("The variable name is {}", var2_ref, var2); this is error as we already have a mutable borrow of var3 in form f var2_ref in line 88 so we cannot have an immutable borrow in println statement
    //so i make sure i resolve this by referrencing var3 to another immutable borrow here i have dereference the mutable referefrence var2_ref which had the ability to be  mutable reference
    //it first took &mut var2 and then it took  &mut var3
    let var4 = *var2_ref;
    println!("The values are - {:?}", (var2_ref, var4));
}
