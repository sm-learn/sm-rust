// Methods, Implementation, static methods 

struct Point {
    x: f64,
    y: f64,
}

//Implementation of Point 
impl Point {
    //Two static methods to create a new Point instance
    fn origin() -> Point  {
        Point { x: 0.0, y: 0.0 }
    }
    
    //Another static method to create apoint from two args
    fn new(x: f64, y:f64) -> Point {
        Point {x: x, y: y}
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    //This is an instance method since it has a reference to self
    // as an argument
    fn area(&self) -> f64 {
        // self gives access to all the structs via the dot operator
        let Point {x: x1, y: y1} = self.p1;
        let Point {x: x2, y: y2} = self.p2;
        
        //Area is the abs of the product of the two points
        ((x1 - x2) * (y1 - y2)).abs()
        
    }
    
    fn perimeter(&self) -> f64 {
        let Point {x: x1, y: y1} = self.p1;
        let Point {x: x2, y: y2} = self.p2;
        
        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
        
    }
    //This method requires the object to be mutable
    fn translate(&mut self, x:f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;
        
        self.p1.y += y;
        self.p2.y += y;
    }
}

//Pair owns two heap allocated integers
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    fn destroy(self) {
        let Pair(first,second) = self;
        println!("Destroying Pair({}, {})", first,second);
        
        // `first` and `second` go out of scope and get freed
    }
}

fn main() {
    let rectangle = Rectangle {
        //Static methods are called using double colons
        p1: Point::origin(),
        p2: Point::new(3.0,4.0),
    };
    // Instance methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());
    
    //rectangle.translate(1.0, 0.0);
    
    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };
    
     // Okay! Mutable objects can call mutable methods
    square.translate(1.0, 1.0);
    
    let pair = Pair(Box::new(1), Box::new(2));
    pair.destroy();

}
    
