use List::*;


enum List {
    //Cons: Tuple struct that wraps an element and a pointer to the next node
    Cons(u32, Box<List>),
    // Nil: signifies the the end of the list
    Nil,
}

//Methods can be attached to an enum
impl List {

    //Create a new List
    fn new() -> List {
        //Nil has the type list
        Nil
    }

    // Add a new element to the given list
    fn prepend(self, elem: u32) -> List {
        // Cons has the type list
        Cons(elem, Box::new(self))
    }

    //return the length of this list
    fn len(&self) -> u32 {
        match *self {
            //Base case when list is empty, len is zero
            Nil => 0,
            // get the reference to the tail
            Cons(_,ref tail) =>  1 + tail.len()
        }
    }
    fn stringify(&self) -> String {
        match *self {
            Cons(head, ref tail) => {
                //format! is similar to println! macro but returns
                // a heap allocated string instead of printing it
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!{"Nil"}
            },
        }
    }
}
fn main() {
    // Create an empty linked list
    let mut list = List::new();

    // Prepend some elements
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);

    // Show the final state of the list
    println!("linked list has length: {}", list.len());
    println!("{}", list.stringify());
}
