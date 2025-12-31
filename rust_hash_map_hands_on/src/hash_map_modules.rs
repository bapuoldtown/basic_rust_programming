use std::collections::HashMap;
use std::collections::hash_map::Entry;

pub fn hash_map_methods(){
    //create a mutable hashmap and use entry method
    //now entey methid handles two scenarios - Vacant and Occupied
    //we gonna handle using match in Rust
    let mut a_hash_map: HashMap<String, i32> = HashMap::new();
    match a_hash_map.entry("Hello".to_string()){
        Entry::Vacant(v) =>{
            v.insert(100);
        }
        Entry::Occupied(_) => {
            println!("The key already exixts");
        }

    }

    //Now whwn next time it updates key we should be able to retrieved the values of key
    match a_hash_map.entry("Hello".to_string()){
        Entry::Occupied(o) =>{
            println!("The value with key is present and its {}", o.get());
        }
        Entry::Vacant(_)=>{
            println!("No key is present bro");
        }
    }

    let m3: HashMap<&str, i32> = HashMap::from([("a", 1), ("b", 2)]);
    println!("from: m3={:?}", m3);
    // ------------------------------------------------------------
    // 2) INSERT / UPDATE
    // ------------------------------------------------------------
    // insert(k, v) -> Option<V>
    let mut m1:HashMap<String, i32> = HashMap::new();
    let ins_cond: Option<i32> = m1.insert("Amigos".to_string(), 10);
    println!("{:?}", m1);
    // entry(k) -> Entry<K, V>
    // or_insert(v) -> &mut V
    //using entry api kind of default dict in python
    let vref: &mut i32 = m1.entry("Amigos_updated".to_string()).or_insert(500);
    println!("The value is {}", vref);
    //Now modify the value of key -Amigos_updated now updated
    *vref = *vref + 200;
    println!("The modified hash map is {:?}", m1);



}