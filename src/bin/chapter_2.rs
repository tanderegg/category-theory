extern crate rand;

use std::time::Duration;
use std::thread;
use std::collections::HashMap;

use rand::Rng;

fn unit<T>(_: T) -> () {}

struct Memoize<T, U, F: FnMut(T) -> U> {
    data: HashMap<T, U>,
    f: F
}

impl<T, U, F> Memoize<T, U, F>
    where T: std::hash::Hash + std::cmp::Eq + std::marker::Copy,
          U: std::marker::Copy,
          F: FnMut(T) -> U {
    fn new(f: F) -> Self {
        Memoize {
            data: HashMap::new(),
            f: f
        }
    }

    fn call(&mut self, x: T) -> U {
        if !self.data.contains_key(&x) {
            self.data.insert(x, (self.f)(x));
        }
        return self.data[&x]
    }
}

fn slow_func(x: u64) -> u64 {
    let mut y = 0;
    for _ in 0..10 {
        y = y + x;
        thread::sleep(Duration::from_millis(100));
    }
    return y
}

fn factorial(x: u64) -> u64 {
    match x {
        0 => 0,
        1 => 1,
        x => x * factorial(x-1)
    }
}

fn f_bool(_: ()) -> bool {
    println!("Hello!");
    return true;
}

fn f_static(x: u64) -> u64
{
    static mut Y: u64 = 0;

    unsafe {
        Y += x;
        return Y;
    }
}

fn main() {
    println!("Unit function of 5 equals unit?: {}", unit(5) == ());

    // Exercise 1
    let mut memoized_slow_func = Memoize::new(slow_func);
    println!("Value of slow_func: {}", memoized_slow_func.call(5));
    println!("Value of memoized slow_func again: {}", memoized_slow_func.call(5));

    // Exercise 2
    let mut rng = rand::thread_rng();
    let mut memoized_gen_u64 = Memoize::new(|_| { rng.gen::<u64>() });
    println!("Random number: {}", memoized_gen_u64.call(1));
    println!("Random number again: {}", memoized_gen_u64.call(1));
    println!("Another random number: {}", memoized_gen_u64.call(2));

    // Exercise 3
    let seed_1 = [1, 2, 3];
    let seed_2 = [2, 3, 4];
    {
        let mut memoized_seed_rng = Memoize::new(|x| {
            let mut rng: rand::StdRng = rand::SeedableRng::from_seed(x);
            rng.gen::<u64>()
        });

        println!("Seeded random number: {:?}", memoized_seed_rng.call(&seed_1));
        println!("Seeded random number again: {:?}", memoized_seed_rng.call(&seed_1));
        println!("Another seeded random number: {:?}", memoized_seed_rng.call(&seed_2));
    }

    // Exercise 4
    println!("Factorial of 100: {}", factorial(20));
    let mut memoized_factorial = Memoize::new(factorial);
    println!("Factorial of 100 again: {}", memoized_factorial.call(20));

    // Have to pass unit paramenter to satisfy type constraints!
    println!("f_bool: {}", f_bool(()));
    println!("f_bool again: {}", f_bool(()));
    let mut memoized_f_bool = Memoize::new(f_bool);
    println!("f_bool memoized: {}", memoized_f_bool.call(()));
    println!("f_bool memoized again: {}", memoized_f_bool.call(()));

    println!("f_static: {}", f_static(64));
    println!("f_static again: {}", f_static(64));
    let mut memoized_f_static = Memoize::new(f_static);
    println!("f_static memoized: {}", memoized_f_static.call(64));
    println!("f_static memoized again: {}", memoized_f_static.call(64));

    // Exercise 5
    fn identity(x: bool) -> bool {
        x
    }

    fn not(x: bool) -> bool {
        !x
    }

    fn f_true(_: bool) -> bool {
        true
    }

    fn f_false(_: bool) -> bool {
        false
    }

    println!("Idenitiy of true: {}", identity(true));
    println!("Not true: {}", not(true));
    println!("True of true: {}", f_true(true));
    println!("False of true: {}", f_false(true));
}
