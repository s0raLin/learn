trait A {}
trait B {}
trait C {}
trait D {}

fn foo<T, K, R>(a: T, b: K, c: R)
where
    T: A,
    K: B + C,
    R: D,
{
}
fn main() {}
