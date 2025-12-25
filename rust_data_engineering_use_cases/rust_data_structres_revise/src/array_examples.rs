/*
Arrays
An array is a collection of a fixed number of elements of any single type T. The array type is [T, n], where T is any type, and n is the fixed number of elements. That is, the array’s size, which is known at compile time.

There are two ways to create arrays:

By listing each element (such as [A, B, C], where A, B, and C are elements of the same type).

By copy-repetitions, which is declaring an element and the number of copies to do of that element (such as [A; 3]). The element type must implement the Copy trait.

Here are some examples of array instantiation:
 */
pub fn array_example(){
   let mut arr1:[f32;3]=[1.0, 2.0,3.5];
 //array can be indexed and same can be mutated simply and straightforwardly
   
   println!("The unchanged array is : {:?}", arr1);
   
   {
      arr1[0]=4.5;
      println!("The changed array is : {:?}", arr1);

   }
   println!("The first element of arr1 is: {:?}", arr1);
   //Default trait for the arrays in Rust
   //Rust implements Debug trait for arrays involved array wity length till 32
   let arr2 =[0;32];
   println!("The array with default values is : {:?}", arr2);
   let arr3 =[0i8;33];
   println!("The array with default values is : {:?}", arr3);
   //slicing array
   let arr4:[i32;5]=[10,20,30,40,50];
   let slice_arr4=&arr4[1..=4];
   println!("The sliced array is :{:?}", slice_arr4);
   //muttable reference slicing to base array
   let mut arr5:[i32;5]=[100,200,300,400,500];
   
   {
      {
         let slice_arr5=&mut arr5[0..2];
         slice_arr5[0]=111;
         slice_arr5[1]=222;
         println!("The sliced mutable array is :{:?}", slice_arr5);

      }
      
      //
      //If we write this let slice_arr6=&mut arr5[0..2]; its an error as rust does not allow miltple write ownership so we can either changed to shared borrowing i.r read only
      //i kept a mutable referrence inside a scope and a immutable referrence in another scope let the territoris be different :)
      {
         let slice_arr6=&arr5[2..5];
         //println!("The sliced mutable array is :{:?} and {:?}", slice_arr5, slice_arr6);
      }
      //more concepts on iterators on an array
      let arr7:[i32;3] = [10, 20,30];
      let index_arr:[i32;3] = [0,1,2];
      for i in index_arr.iter(){

         //println!("The element is : {:?}", arr7[i as usize]);// this will give you an error because index of array has to be of usize type, now we give i as usize now this is also incorrect the reason is i is an refereence to the array iterators we have to dereference it and then concert to its respectice type brother :)
         println!("The element is : {:?}", arr7[*i as usize]);
      }
      //now brother what happened here is we are using a index array which is of i32 so we had to typecaste to use as an index.
      //Now brother lets use our index array of usize then we will safe from type casting
      let index_arr2:[usize;3]=[0,1,2];
      //or simply we can write let index_arr2=[0,1,2]; Rust will infer the type itself :)
      for j in index_arr2.iter(){
         println!("The element is : {:?}", arr7[*j]);
      }      
      
      
   }
   println!("The sliced mutable array is :{:?}", arr5);
   /*
         The sliced patterns
         One of the most useful constructs of Rust is its ability to match patterns.
         The slice pattern extracts elements out of an array:
         However, we can also match using a slice pattern!

    */
    let arr8=[1.5, 2.5];
    let [a,b]=arr8;
    println!("The values are :{} and {}",a,b);//extracts element scenarios
    //lets now implemet match pattern case below
    let v = vec!["apple",];

   // Convert Vec<&str> to &[&str]
   let slice = v.as_slice();//to implement slice pattern we will have to borrow as slice first brother
   match slice{
      [] => println!("The slice is empty"),
      [A] => println!("The slice has one element: {}", A),
      [A, B] => println!("The slice has two elements: {} and {}", A, B),
      _ => println!("The slice has more than two elements"),
   }

   



}
 