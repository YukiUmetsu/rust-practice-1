
trait Duplicatable {
    fn dupl(&self) -> String;
}

impl Duplicatable for String {
   fn dupl(&self) -> String {
        format!("{0} {0}", *self)
   }
}

impl Duplicatable for i32 {
    fn dupl(&self) -> String {
        format!("{}", *self*2)
    }
}

// static dispatch, convert type at compile time.
fn static_duplicate<T: Duplicatable>(x: &T) {
    println!("{}", (*x).dupl());
}

// dynamic dispatch, convert type and decide which function to call at run time.
fn dynamic_duplicate(x: &dyn Duplicatable) {
    println!("{}", x.dupl());
}

fn main() {
    let a = 42;
    let b = "Hi John".to_string();
    println!("\n-------Static Dispatch------\n");
    static_duplicate(&a);
    static_duplicate(&b);

    println!("\n-------Dynamic Dispatch------\n");
    dynamic_duplicate(&a);
    dynamic_duplicate(&b);
}
