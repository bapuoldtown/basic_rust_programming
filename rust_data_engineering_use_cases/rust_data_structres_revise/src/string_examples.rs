pub fn str_example(){
    let s1: &str="hello";
    let s2: String=String::from("world");
    let s3=format!("{}{}",s1,s2);
    let s4=s2.contains("or");

    let s6 = String::from("Guru Prasad Pattnayak");
    println!("The formatted string is : {}", s4);
    let s5=s3.replace("hello","hi");
    println!("The replaced string is : {}", s5);
    for i in s5.chars(){
        println!("The chars in s5 are : {}", i);
    }
    for i in s5.chars().rev(){
        println!("The rev chars in s5 are : {}", i);
    }

    for i in s6.chars().rev(){
        println!("The rev chars in s6 are : {}", i);
    }
    //let declare a string and convert to uppercase
    let s7=String::from("rust programming language");
    for i in s7.chars(){
        println!("The chars in s7 are : {}", i.to_uppercase());
    }

    let mut s8=String::from("guru krisha sujita");
    for i in (0..s8.len()){
        println!("The character is {}", s8.chars().nth(i).unwrap());
    }
    for i in (0..s8.len()){
        println!("The byte is {}", s8.as_bytes()[i]);
    }
    //Use Some to retrieve the characted and dispay
    //Some(n) returns either a true value or false i. nNone Rust gives a match of succesand exception
    for i in 0..=s8.len(){
        if let Some(c) = s8.chars().nth(i){
            println!("The characted at index is {}", c);
        }
        else{
            println!("No character at index {}", i);
        }
    }
   
    
}