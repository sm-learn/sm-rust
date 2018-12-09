//! For pointers, a distinction needs to be made between destructuring and dereferencing as they are different concepts
//! which are used differently from a language like C.
//! Dereferencing uses *
//! Destructuring uses &, ref, and ref mut
//!

fn main() {
    //assign a reference of type u32. The & signifies there is a reference being attached.
    let reference = &4;
    
    match reference {
        // If the reference's is pattern matched adainst &val, it results in a comparison 
        // like: &i32, &val
        &val => println!("Got a value via destructuring: {:?}",val),
    }
    match *reference {
        val => println!("Got a value vis dereferencing: {}", val),
    }
    // What happens when we start without a reference
    let not_a_reference = 3;
    
    let value = 6;
    
    match value {
        ref r => println!("Got a reference to a value: {:?}", r),
    }
    
    let mut mutated_val = 56;
    
    match mutated_val{
        ref mut m => {
            // Got a reference, now got to dereference it
             *m += 10;
             println!("We added 10: to mutated value: {:?}", m);
        }
    }
    
}
