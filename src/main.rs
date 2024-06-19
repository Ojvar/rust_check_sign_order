use std::{
    collections::HashMap,
    io::{self, Write},
};

struct Stack<T> {
    elements: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Self {
            elements: Vec::new(),
        }
    }

    fn push(&mut self, item: T) {
        self.elements.push(item);
    }

    fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    fn peek(&self) -> Option<&T> {
        self.elements.last()
    }

    fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    fn size(&self) -> usize {
        self.elements.len()
    }
}

fn check_order(input_str: &str) -> bool {
    let mut stack: Stack<char> = Stack::new();
    let signs = HashMap::from([(')', '('), (']', '['), ('}', '{')]);

    for c in input_str.chars() {
        if let Some(&open_bracket) = signs.get(&c) {
            match stack.pop() {
                Some(top) if top == open_bracket => (),
                _ => return false,
            }
        } else {
            stack.push(c);
        }
    }
    stack.is_empty()
}

fn main() {
    let mut input_str = String::new();
    io::stdout().flush().expect("Failed to flush stdout");
    println!("Enter your input string: ");
    io::stdin()
        .read_line(&mut input_str)
        .expect("Failed to read line");

    let result = check_order(input_str.as_str());
    println!("Result is {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correct_sequence() {
        let input_str = "[](){}[{}][()][{()}]({[]})";
        let result = check_order(input_str);
        assert_eq!(result, true);
    }

    #[test]
    fn test_incorrect_sequence() {
        let input_str = "[}(){}[{}][()][{()}]({[]})";
        let result = check_order(input_str);
        assert_eq!(result, false);
    }
}
