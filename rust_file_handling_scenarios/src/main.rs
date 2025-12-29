use std::fs;
use std::fs::File;
use std::io::{self, BufRead};

fn file_read_result_Error(file_path: &str) -> Result<String, std::io::Error>{
    let contents = fs::read_to_string(file_path)?;
    Ok(contents)

}

fn file_read_push_vector(file_path: &str) -> Result<Vec<String>, std::io::Error>{
    let contents= fs::read_to_string(file_path)?;
    let mut line_vector:Vec<String> = Vec::new();
    let mut line_vector2:Vec<String> = Vec::new();
    for line in contents.lines(){
        line_vector.push(line.to_string());
    }
    for line in contents.split("\n"){
        line_vector2.push(line.to_string());
    }
    println!("The second factor is {:?}", line_vector2);
    return Ok(line_vector);
//This reads a file content convert into list of strings
}

fn file_read_print_wanted_line(file_path: String, wanted_line: String){
    println!("enyerrin function to open file path:{}", file_path);
    let file_content:String = fs::read_to_string(file_path)
        .expect("Failed to read the file");
    for line in file_content.lines(){
        if line.contains(&wanted_line){
            println!("Found the line: {}",line);
        }
    }
}

fn file_read_lines_break_words(file_path: &str) -> Result<Vec<String>, std::io::Error>{
    let contents = fs::read_to_string(file_path)?;
    let mut word_vector:Vec<String> = Vec::new();
    for line in contents.split("\n"){
        for word in line.split_whitespace(){
            word_vector.push(word.to_string());
        }
    }
    //Lets create an iter and use  map bruv below
    let word_vector3:Vec<String>= contents.lines().flat_map(|line| line.split_whitespace()).map(|l| l.to_string()).collect();
    println!("The word vector using iter and map is {:?}", word_vector3);
    //briv pasre files capture only second word do not capture hello
    let word_vector4=contents.lines().filter_map(|line| line.split_whitespace().nth(1)).map(|l| l.to_string()).collect::<Vec<String>>();
    println!("The word vector using iter and map is {:?}", word_vector4);
    Ok(word_vector)

}

fn file_write_string_vector(file_path: &str, content_string: Vec<String>) -> Result<(), std::io::Error>{
    fs::write(file_path, content_string.join("\n")).expect("Failed to create new files");
    Ok(())
}
fn main() {
    println!("Hello, world!");
    let file_path = "./rust_test_file.txt";
    let file_content:String = fs::read_to_string(file_path)
        .expect("Failed to read the file");
    println!("File content: {}", file_content);
    for line in file_content.lines(){
        if line.contains("Lets open it bruv"){
            println!("Found the line: {}",line);

        }
    }

    //Read file and print line by line
    let file_ob=File::open("./rust_test_file.txt").expect("Failed to open file");
    //Create a buffered read streamer on the same file object
    let buf_reader=io::BufReader::new(file_ob);
    for line in buf_reader.lines(){
        println!("{}",line.expect("Could not read line"));
    }
    //
    //let file_ob=File::open("./rust_test_file1.txt").expect("Failed to open file Bruv");
    //Create a buffered read streamer on the same file object
    //let buf_reader=io::BufReader::new(file_ob);
    //for line in buf_reader.lines(){
        //println!("{}",line.expect("Could not read line"));
    //}
    //Write files to a new file
    let content:String = String::from("Hello this is a new file created by Rust");
    fs::write("./rust_test_file_write.txt", content).expect("Failed to create new files");
    //calling function to read wanted files
    file_read_print_wanted_line("./rust_test_file.txt".to_string(), String::from("Lets open it bruv"));

    let content_result = file_read_result_Error("./rust_test_file.txt");
    println!("The content result is {:?}",content_result);

    let ret_vector = file_read_push_vector("./rust_file_read_line1.txt");
    println!("The returned vector is {:?}",ret_vector);

    let ret_word_vector = file_read_lines_break_words("./rust_file_read_line1.txt");
    println!("The returned word vector is {:?}",ret_word_vector);
    //write a vector of sttrings to a file and return Result with unit type and Error strinf
    let word_vector4=vec![String::from("Guru Prasad"), String::from("Krisha Kriti"), String::from("Sujita Aparajita")];
    let write_result = file_write_string_vector("./rust_file_write_vector.txt", word_vector4);
    println!("The write result is {:?}",write_result);
}
