use std::collections::HashMap;

fn random_stat_buff() -> usize{
    50
}

pub fn hash_map_entry_api(){
    //we gonna do advance hands on with entry API used for 
    let mut player_stats = HashMap::new();
    // insert a key only if it doesn't already exist
     player_stats.entry("health").or_insert(100);
     // insert a key using a function that provides a new value only if it
    // doesn't already exist
    player_stats.entry("defence").or_insert_with(random_stat_buff);
    println!("The hash map is {:?}", player_stats);
    // update a key, guarding against the key possibly not being set
    let value = player_stats.entry("attack").or_insert(100); //returns a mutable refrrence
    //lets mutate by accessing the referrence bruv
    *value += random_stat_buff() + 200;
    println!("The mutated hash map is {:?}", player_stats);
    //modify the value in place if the key is presentr else
    //objective change the value if key is present using and_modify
    //then use or_insert to push the value 
    player_stats.entry("mod_key").and_modify(|mod_key| *mod_key += 200).or_insert(50);
    println!("The newly updated key is {:?}", player_stats);
    //now run it again it sould modify the key value in line
    player_stats.entry("mod_key").and_modify(|mod_key| *mod_key += 200).or_insert(50);
    println!("The newly updated key is {:?}", player_stats);

    //common methods of hashmap
    let mut a_hashmap = HashMap::new();
    a_hashmap.insert(1, "a" );
    let a_len = a_hashmap.len();

    println!("The len of hash map is {}", a_len);
    //flush the hash map length is -
    a_hashmap.clear();
    println!("The len of hash mao is {}", a_hashmap.len());
    //ceheck hash map is empty
    assert!(a_hashmap.is_empty());
    //assert!(!a_hashmap.is_empty())
    //contains_key contains_key<Q>(&self, k: &Q) -> bool
    let mut map = HashMap::new();
    map.insert(1, "a");
    map.insert(2, "guru");
    //Returns true if the map contains a value for the specified key.
    println!("The get value is {}",map.contains_key(&1));
    //pub fn get<Q>(&self, k: &Q) -> Option<&V> if not return s None
    //Returns a reference to the value corresponding to the key.
    let mut mut_map = HashMap::new();
    mut_map.insert(1, "a");
    assert_eq!(mut_map.get(&1), Some(&"a"));
    assert_eq!(mut_map.get(&2), None);
    //fn get_mut<Q>(&mut self, k: &Q) -> Option<&mut V>
    /*
        where
            K: Borrow<Q>,
            Q: Hash + Eq + ?Sized,
        Returns a mutable reference to the value corresponding to the key.

        The key may be any borrowed form of the map’s key type, but Hash and Eq on the borrowed form must match those for the key type.
     */

    let mut mut_map1=HashMap::new();
    mut_map1.insert("a".to_string(), 10);

    if let Some(x) = mut_map1.get_mut("a"){
        *x += 200;
    }
    //assert_eq!(mut_map1.get("a".to_string()), &210);
    println!("The hash map is {:?}", mut_map1);




}