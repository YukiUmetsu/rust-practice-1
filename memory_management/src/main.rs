#[warn(unused_mut)]
use std::rc::Rc;

#[derive(Debug)]
struct Person {
    name: String
}

#[derive(Debug)]
struct Dog <'l>{
    name: String,
    owner: &'l Person
}

struct Car {
    brand: Rc<String>
}

impl Car {
    fn new(brand: Rc<String>) -> Car { Car{ brand: brand } }
    fn drive(&self) {
        println!("{} is driving", &self.brand);
    }
}

fn main() {
    // Ownership change
    let i = 5;
    let j = i;      // copying instead of moving ownership because it is primitive type (cheap to copy)
    println!("{}", i);
    println!("{}", j);

    let v = vec![0,1,2,3,4,5,6,7,8,9];
    let w = v;          // value borrowed here after move. change of ownership from v to w.
    println!("{:?}", w);
    // println!("{:?}", v);     <- this will error

    let foo = |v: Vec<i32>| -> Vec<i32> {
        v
    };
    let v = foo(w);         // v ownership transferred to foo function. then transferred back to let v.
    println!("{:?}", v);

    // Borrowing
    let mut vec1 = vec![0,1,2,3,4,5,6,7,8,9];
    for i in &vec1 {
        println!("i {}", i);       // borrowing as immutable
        // v.push(*i);             <-- error because i was borrowed as immutable
    }


    // Life time
    let p1 = Person { name: String::from("John")};
    let d1 = Dog { name: String::from("Max"), owner: &p1 };
     // without lifetime of person reference, it errors because if person becomes invalid, owner might point to invalid memory
    println!("{:?}", d1);


    // reference counted variables
    let brand = Rc::new(String::from("BMW"));
    println!("pointers: {}", Rc::strong_count(&brand));
    {
        let car = Car::new(brand.clone());
        car.drive();
        println!("pointers: {}", Rc::strong_count(&brand));
    }
    println!("pointers: {}", Rc::strong_count(&brand));
}
