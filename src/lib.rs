#[cfg(test)] // only compile when you run `cargo test`
mod tests;

pub mod string {
    /// Reverse a string
    pub fn reverse<'a>(s: &'a str) -> String {
        s.chars().rev().collect::<String>()
    }
}
