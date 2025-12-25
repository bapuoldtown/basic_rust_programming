pub fn vector_example(){
    /*
    However, above and beyond that distinction, vectors are dynamic arrays, meaning that:

Vectors are declared as generics (such as vec<T>). This also means that a vector, like an array, can hold only one type.
Vectors are a particular kind of structs.
Vectors allocate data on the heap. This is useful to know if we have memory restrictions in an embedded environment.
We can make vectors grow or shrink in size with specific commands.
Arrays, on the contrary, are not dynamic. We have to declare their size beforehand.
     */
    let mut vec1:Vec<i32>=Vec::new();
    let mut vec2 = vec![1,2,3,4,5];
    let vec3=vec![10,20,50];
    vec2.push(1);
    println!("The vector vec2 after push is : {:?}", vec2);
    //exhubiting python extend
    vec2.extend(&vec3);
    println!("The vector vec2 after extend is : {:?}", vec2);
    vec2.append(&mut vec![100,200]);
    //vec2.append(&mut vec3);
    println!("The vector vec2 after append is : {:?}", vec2);
    //vec2.append(&mut vec3);
    //println!("The vector vec2 after append is : {:?}", vec2);
    //Lets do some Vec map and filter bruv
    let vect1:Vec<i32>=vec![1,2,3,4,5,6,7,8,9,10];
    let vect2:Vec<i32> = vect1.iter().map(|item| item*2).collect();
    println!("The new vector is {:?}", vect2);
    //Now filter vector referrence via some condition and then map a function
    //iter returnd references of vec elements and filter destrrucres references to reference twice and if we give collect then the capturinf container should be refernce
    //but if our target container is i32 then put copied chaining before collecting i will show below
    let vect3:Vec<&i32>=vect1.iter().filter(|&&x| x%2==0).collect();
    //we can dereference like this as well
    let vect4:Vec<&i32>=vect1.iter().filter(|it| **it%2==0).collect();
    let vect5:Vec<i32>=vect1.iter().filter(|it| **it%2==0).copied().collect();
    println!("The filtered vector is {:?}", vect3);
    println!("The filtered vector with copied is {:?}", vect4);
    println!("The filtered vector with copied is {:?}", vect5);
    //Mode 1: iter() (borrow, read-only) ✅
    //Mode 2: into_iter() (move/consume) ✅
    /*
    Items are i32 (owned values)

        v is consumed (can’t use after)

        Scenario 2A: Map without needing copied
     */
    let v_doubled = vec![1,2,3,4];

    let doubled: Vec<i32> = v_doubled.into_iter()
        .map(|x| x * 2)
        .collect();
    println!("Doubled: {:?}", doubled);
    //original v_doubled is no more valid as into_iter changes ownsership
    //error - println!("Original vector after into_iter is consumed and cannot be used: {:?}", v_doubled);
    /*
         Mode 3: iter_mut() (borrow mutably, modify in place) ✅

            Items are &mut i32

            You can change elements inside v

            Scenario 3A: Multiply every element by 10 (in-place)
     */
    let mut v_mut = vec![1, 2, 3, 4];
    for item in v_mut.iter_mut() {
        *item *= 10; // Dereference and modify
    }
    println!("Modified v_mut: {:?}", v_mut);
    //Bruv now lets do some exampl;es using for_each
    //for_each basically you can relate to doing inline 
    //for_each is a terminal step like a for loop — it consumes the iterator and returns (); it’s used for side effects, not for building collections.
    //for example think of a situation where we need to increment an counter basee on some values of a iterator we can use convinient for_each instead of for
    //it returns nothing so do not use to store its return types
    let mut count_event = 0;
    let numbers = vec![1,2,3,4,5,6,7,8,9];
    numbers.iter().filter(|x| **x%2==0).for_each(|_| count_event +=1);
    println!("The count of even numbers is : {}", count_event);
    //
}