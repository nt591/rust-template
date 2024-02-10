struct Calculator;

impl Calculator {
    pub fn add(x: usize, y:usize) -> usize {
        x + y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_works_as_expected() {
        assert_eq!(Calculator::add(4, 5), 9);
    }
}
