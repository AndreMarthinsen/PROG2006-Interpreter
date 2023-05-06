//! This module implements a traditional stack data structure with the
//! usual top, pop, and push methods along with iterator methods.
//!
//! The Stack is represented as an enum with two variants:
//! - Empty: represents an empty stack
//! - Top: represents a non-empty stack and holds the current value on top of
//!        the stack, a pointer to the bottom of the stack, and the current
//!        size of the stack.
//!
//! The Stack object provides methods to push, pop, and retrieve the top
//! element without popping it off the stack. The object also provides
//! methods to display all contents of the stack, check if the stack is
//! empty, retrieve the element count of the stack, and create iterators
//! over the stack's contents.
//!
//! Additionally, this module provides the implementation of StackIter
//! and the ability to create a new stack from an iterator using `.collect()`.
//!
//! # Examples
//!
//! ```
//! use stack::Stack;
//!
//! let mut stack = Stack::new();
//! assert_eq!(Stack::Empty, stack);
//!
//! stack.push(5);
//! assert_eq!(5, *stack.top().unwrap());
//! assert_eq!(1, stack.size());
//!
//! let top = stack.pop().unwrap();
//! assert_eq!(5, top);
//!
//! assert_eq!(true, stack.is_empty());
//! ```


use std::fmt::{Debug, Display};

#[derive(PartialEq, Clone, Debug)]
/// Stack implements a traditional stack data structure with the
/// usual top, pop, and push methods along with iterator methods.
pub enum Stack<T: Clone + Display + Debug> {
    Empty,
    Top(T, Box<Stack<T>>, usize)
}



impl<T:Clone + Display + Debug> Stack<T> {

    /// Prints the contents of the stack top to bottom left to right
    pub fn display_all_contents(&self) {
        let mut output = String::from("");
        self.iter().for_each(|x| {
            output.push_str(&x.to_string());
            output.push(' ');
        });
        println!("{}", output);
    }

    /// Constructs a new empty stack
    ///
    /// ```
    /// let mut stack = stack::Stack::new();
    /// assert_eq!(Stack::Empty, stack);
    /// ```
    ///
    pub fn new() -> Self {
        Stack::Empty
    }

    /// Pushes a new value T onto the stack.
    ///
    /// # Arguments
    /// `val`- An object of type T to be pushed onto the stack.
    ///
    /// ```
    /// let mut stack = stack::Stack::new();
    /// stack.push(5);
    /// assert_eq!(5, *stack.top());
    /// ```
    ///
    pub fn push(&mut self, val: T) {
        match self {
            Stack::Empty=> *self = Stack::Top(val, Box::new(Stack::Empty), 1),
            Stack::Top(_, _, size) => {
                let new_size = *size + 1;
                let old_stack = Box::new(std::mem::replace(self, Stack::Empty));
                *self = Stack::Top(val, old_stack, new_size)
            }
        }
    }

    /// Pops the top value off of the stack and returns the value.
    ///
    /// # Returns
    ///
    /// Some(T) if the stack is non-empty,
    /// None otherwise.
    ///
    /// ```
    /// let mut stack = stack::Stack::new();
    /// stack.push(5);
    /// let top = stack.pop().unwrap();
    ///
    /// assert_eq!(top, 6);
    /// assert_eq!(Stack::Empty, stack);
    /// ```
    ///
    pub fn pop(&mut self) -> Option<T> {
        match std::mem::replace(self, Stack::Empty) {
            Stack::Empty => None,
            Stack::Top(val, bottom, _) => {
                *self = *bottom;
                Some(val)
            }
        }
    }

    /// Returns a reference to the top element without popping it off the stack.
    ///
    /// # Returns
    /// Some(T) if stack is non-empty,
    /// None otherwise.
    ///
    pub fn top(&self) -> Option<&T> {
        match self {
            Stack::Top(val, _ ,_) => Some(val),
            _ => None
        }
    }

    /// Checks if the stack is empty
    ///
    /// # Returns
    ///
    /// true if empty, false otherwise;
    ///
    /// ```
    /// let stack = Stack::new();
    /// assert_eq!(true, stack.is_empty());
    ///
    /// ```
    pub fn is_empty(&self) -> bool {
        match self {
            Stack::Empty => true,
            _ => false
        }
    }

    /// Retrieves the element count of the stack
    ///
    /// # Returns
    ///
    /// The amount of elements currently on the stack
    ///
    pub fn size(&self) -> usize {
        match self {
            Stack::Empty => 0,
            Stack::Top(_, _, size) => *size
        }
    }

    /// Returns an iterator over the stacks contents.
    pub fn iter(&self) -> impl Iterator<Item=&T> {
        self.into_iter()
    }
}



/// Implements StackIter
pub struct StackIter<'a, T:Clone + Display + Debug> {
    stack: &'a Stack<T>,
}

/// Implements IntoIterator for Stack, allowing the creation of an
/// iterator for the Stack object.
impl<'a, T:Clone + Display + Debug> IntoIterator for &'a Stack<T> {
    type Item = &'a T;
    type IntoIter = StackIter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        StackIter { stack: self }
    }
}

/// Implements FromIterator for Stack, allowing the use of .collect() to create
///  a new stack from an iterator.
impl<T:Clone + Display + Debug> FromIterator<T> for Stack<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut stack = Stack::Empty;
        for val in iter {
            stack.push(val);
        }
        stack
    }
}

/// Implements Default value for a stack. Using Stack::new() will use the
/// default to initialize the Stack object as Stack::Empty.
impl<T:Clone + Display + Debug> Default for Stack<T> {
    fn default() -> Self {
        Stack::Empty
    }
}

/// Implements an Iterator for StackIter
impl <'a, T:Clone + Display + Debug> Iterator for StackIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        match self.stack {
            Stack::Top(val, bottom,_) => {
                self.stack = bottom;
                Some(val)
            }
            Stack::Empty => {
                None
            }
        }
    }

}
