trait Bar {
    
}
trait Foo {
    fn foo1(&self) {
        println!("Hello1")
    }
    fn foo2(&self) {
        println!("Hello2")

    }
}
struct Doo;

impl Foo for Doo {}
fn dynamic_dispatch(t: &dyn Foo) {
    t.foo1();
    t.foo2();
}

fn main() {
    let doo = Doo;
    doo.foo1();
    doo.foo2();

    dynamic_dispatch(&doo);

}