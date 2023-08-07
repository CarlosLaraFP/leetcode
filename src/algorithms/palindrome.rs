/// Given an integer x, return true if x is a palindrome and false otherwise.
///
pub fn is_palindrome(x: i32) -> bool {
    let input = x.to_string();
    let clone = x.to_string();
    let mut reversed = clone.chars().rev();
    for c in input.chars() {
        if c != reversed.next().unwrap() {
            return false;
        }
    }
    true
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first() {
        assert!(is_palindrome(121));
    }

    #[test]
    fn second() {
        assert!(!is_palindrome(-121));
    }

    #[test]
    fn third() {
        assert!(!is_palindrome(10));
    }
}