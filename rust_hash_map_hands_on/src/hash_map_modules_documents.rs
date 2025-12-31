use std::collections::HashMap;
use std::collections::hash_map::Entry;


pub fn hash_map_documentation_code(){
    let mut book_reviews = HashMap::new();
    //use insert for beginning
    book_reviews.insert("Adventures of Huckleberry Finn".to_string(), "My favorite book.".to_string(),);

    book_reviews.insert(
    "Grimms' Fairy Tales".to_string(),
    "Masterpiece.".to_string(),
);

    book_reviews.insert(
    "Pride and Prejudice".to_string(),
    "Very enjoyable.".to_string(),
);
    book_reviews.insert(
        "The Adventures of Sherlock Holmes".to_string(),
        "Eye lyked it alot.".to_string(),
    );

    //check key is present in the hashmap
    book_reviews.insert(String::from("test"), String::from("value"));

    if !book_reviews.contains_key("test1"){
        println!("The key is not present");
    }

    println!("The hash map is {:?}", book_reviews);
    //iterating over key value pairs bruv
    for (k, v) in book_reviews.iter(){
        println!("The key is {} with value {}", k,v)

    }
    //remove the key bruv with key test
    if let Some(v) = book_reviews.get("test"){
        println!("The key is presnt bruv");
        book_reviews.remove("test");
        println!("The updated hashmap is {:?}", book_reviews);
    }
    


}