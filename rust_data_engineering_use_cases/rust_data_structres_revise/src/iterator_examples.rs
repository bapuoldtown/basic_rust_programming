pub fn iter_examples(){
    //We will focus on iterators inly bru
    //deal with rust compoements that return an interators basically
    let v = vec![1,2,3];
    let mut v_mut = vec![1,2,3];
    v.iter();      // Item = &i32  (borrow)
    v.into_iter(); // Item = i32   (move/consume)
    v_mut.iter_mut();  // Item = &mut i32 (mutate)
    let obj=(1..=10);
    println!("The object is : {:?}", obj);
    "aa bb".split_whitespace(); // &str iterator
    //transform range iterator ob to multiply elements by 2 bruv
    let doubled_vector:Vec<i32>=(1..=5).map(|it| it*2).collect();
    println!("The doubled vector is : {:?}", doubled_vector);
}