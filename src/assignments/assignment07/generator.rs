//! Generators
//!
//! HINT: Look at the `generator_grade.rs` file to see how the generator is used.

/// Yielded value. It can be either a value or a stop signal.
enum Yielded<T> {
    Value(T),
    Stop,
}

/// Generator
/// - You can call `next()` method to get the next value.
/// - The generator should stop when it yields `Yielded::Stop`.
///
/// Reference:
/// - [Python generator](https://python-reference.readthedocs.io/en/latest/docs/generator/)
#[derive(Debug)]
pub struct Generator<T, S> {
    state: S,
    f: fn(&mut S) -> Yielded<T>,
}

impl<T, S> Iterator for Generator<T, S> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match (self.f)(&mut self.state) {
            Yielded::Value(v) => Some(v),
            Yielded::Stop => None,
        }
    }
}

/// Returns a generator that yields fibonacci numbers.
///
/// HINT: Consult <https://en.wikipedia.org/wiki/Fibonacci_sequence>
pub fn fib_generator(first: usize, second: usize) -> Generator<usize, (usize, usize)> {
    let mut state = (first, second);
    let f = |state: &mut (usize, usize)| {
        let (a, b) = (state.0, state.1);
        state.0 = b;
        state.1 = a + b;
        Yielded::Value(a)
    };

    Generator { state, f }
}

/// Returns a generator that yields collatz numbers.
///
/// HINT: Consult <https://en.wikipedia.org/wiki/Collatz_conjecture>
pub fn collatz_conjecture(start: usize) -> Generator<usize, usize> {
    let state = start;
    let f = |state: &mut usize| {
        if *state == 1 {
            *state = 0;
            Yielded::Value(1)
        } else if *state == 0 {
            Yielded::Stop
        } else if *state % 2 == 0 {
            let ret = *state;
            *state /= 2;
            Yielded::Value(ret)
        } else {
            let ret = *state;
            *state = 3 * *state + 1;
            Yielded::Value(ret)
        }
    };
    Generator { state, f }
}
