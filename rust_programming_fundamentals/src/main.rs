fn main() {
    println!("Basic If else Example");
    let message=100;
    if message > 50{
        println!("Message is greater than 50");
    } else if message == 50{
        println!("Message is equal to 50");
    }
    else {
        println!("Message is less than 50");
    }

    println!("Usage of expression and if -else combo");
    let express_value = if message >50 {"Guru"} else {"Winnie"};
    println!("The value of express_value is: {}", express_value);

    println!("Switch case using match statement");
    let error_code : i32 = 404;
    let error_message = match error_code {
        200 => "OK",
        404 => "Not Found",
        500 => "Internal Server Error",
        _ => "Unknown Error",
    };
    println!("The error message for code {} is: {}", error_code, error_message);

    println!("Switch case in range using match statement");
    let log_threshold : i32 = 75;
    let log_message = match log_threshold{
        0..50 => "Low Severity",
        60..100 => "Medium Severity",
        120..150 => "High Severity",
        _ => "Critcal Severity",
    };

    println!("The log threshold is {} and the log message is {}", log_threshold, log_message);

    println!("Match with multiple patterns");
    let my_value : i16 = 60;
    let ret_value=match my_value {
        1|2|3 => "Low Value",
        4|5 => "Medium Value",
        6 => "High Value",
        _ => "Unknown Value",

    };

    println!("The return value {}", ret_value);

    println!("Infinite loop usage example");
    let mut counter =0;
    let result = loop {
        counter += 1;
        if counter == 5 {
            break counter *3;
        }
    };
    println!("The result from the infinite loop is: {}", result);

    println!("While loop usage examplw");
    let mut loop_counter = 3;
    while loop_counter !=0 {
        println!("Hello Guri, The counter value is :{}", loop_counter);
        loop_counter -=1;
    };
    println!("For loop usage example if we only mention .. then it takes lower value inclusinve and not upper value");
    for i in 1..5{
        println!("The value of i is: {}", i);
    }
    println!("For loop usage example if we mention ..= then it takes both lower and upper value inclusinve");
    for i in 1..=6{
        println!("The value of i is: {}", i);
    }
    println!("Printing values in reverse order using lower and highr values included threshold");
    for i in (1..=5).rev(){
        println!("The value of i is: {}", i);
    }
    println!("The code block to traverse an array");
    let my_arr = ["Guru","Krisha", "Sujita", "Bapu","Junu","Winnie"];
    for name in my_arr.iter(){
        println!("Name is {}", name.to_uppercase());
    }

    // for with index using enumerate
    let my_arr2: [&str;6]= ["Guru","Krisha", "Sujita","Bapu","Junu","Winnie"];
    for (index, pos) in my_arr2.iter().enumerate(){
        println!("The index is {} and position is {}", index, pos);

    };


}
