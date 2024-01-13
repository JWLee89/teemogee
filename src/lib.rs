pub mod string {
    /// Reverse a string
    pub fn reverse<'a>(s: &'a str) -> String {
        s.chars().rev().collect::<String>()
    }
    pub fn get_longer<'a>(s1: &'a str, s2: &'a str) -> &'a str {
        if s1.len() > s2.len() {
            s1
        } else {
            s2
        }
    }
}