#[derive(Debug, PartialEq)]
pub struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

#[derive(Debug, PartialEq)]
pub struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>
}

impl<T> LinkedList<T> {
    
    pub fn new() -> LinkedList<T> {
        LinkedList {
            head: None
        }
    }

    pub fn push(&mut self, value: T) {
        let new_node = Node {
            value,
            next: self.head.take(),
        };

        self.head = Some(Box::new(new_node));
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.value
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.value)
    }

    pub fn iter(&self) -> Iter<T> {
        Iter {
            // Dereference the head, derefence the Box, return reference to the Node in the Box
            next: self.head.as_ref().map(|node| &**node),
        }
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            next: self.head.as_mut().map(|node| &mut **node),
        }
    }
    
}


pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_ref().map(|node| &**node);
            &node.value
        })
    }
}

pub struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.take().map(|node| {
            self.next = node.next.as_mut().map(|node| &mut **node);
            &mut node.value
        })
    }
}


impl<T> Iterator for LinkedList<T> {
    type Item = T;
    
    fn next(&mut self) -> Option<Self::Item> {
        self.pop()
    }    
}

// Drop is implemented this way to avoid potentially blowing the stack
impl<T> Drop for LinkedList<T> {

    fn drop(&mut self) {
        let mut current_link = std::mem::replace(&mut self.head, None);
        while let Some(mut boxed_node) = current_link {
            current_link = std::mem::replace(&mut boxed_node.next, None);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let list = LinkedList::<f64>::new();
        assert_eq!(list.head.is_none(), true);
    }

    #[test]
    fn test_push_pop() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);
        list.push(3);
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);                
    }

    #[test]
    fn test_peek() {
        let mut list = LinkedList::new();
        list.push(1);
        assert_eq!(list.peek(), Some(&1));
        list.pop();
        assert_eq!(list.peek(), None);        
    }

    #[test]
    fn test_peek_mut() {
        let mut list = LinkedList::new();
        list.push(1);
        list.peek_mut().map(|value| {
            *value = 2;
        });

        assert_eq!(list.peek(), Some(&2));
        list.pop();
        assert_eq!(list.peek(), None);        
    }

    #[test]
    fn test_into_iter() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);

        let mut list_it = list.into_iter();
        assert_eq!(list_it.next(), Some(2));
        assert_eq!(list_it.next(), Some(1));
        assert_eq!(list_it.next(), None);        
    }

    #[test]
    fn test_iter() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);

        let mut list_it = list.iter();
        assert_eq!(list_it.next(), Some(&2));
        assert_eq!(list_it.next(), Some(&1));
        assert_eq!(list_it.next(), None);        
    }

    #[test]
    fn test_iter_mut() {
        let mut list = LinkedList::new();
        list.push(1);
        list.push(2);

        let mut list_it = list.iter_mut();
        list_it.next().map(|value| {
            *value = 3;
        });

        list_it = list.iter_mut();        
        assert_eq!(list_it.next(), Some(&mut 3));
        assert_eq!(list_it.next(), Some(&mut 1));
        assert_eq!(list_it.next(), None);        
        
    }
    
}
