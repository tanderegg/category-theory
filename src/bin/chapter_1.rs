fn identity<T>(x: T) -> T {
    return x
}

fn g_after_f<F: 'static, G: 'static, T, U, V>(g: G, f: F) -> Box<Fn(T) -> V>
    where F: Fn(T) -> U, G: Fn(U) -> V {
    return Box::new(move |x| g(f(x)))
}

fn f(x: i32) -> i32 {
    return x * 2 + 7
}

fn main() {
    let mut x = 124;
    println!("Identity of x ({}): {}", x, identity(x));
    x = 0;
    println!("Identity of x ({}): {}", x, identity(x));
    println!("f of 5: {}", f(5));
    println!("f of identity of 5: {}", g_after_f(identity, f)(5));
}
