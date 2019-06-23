pub mod data {

    #[derive(Debug)]
    pub struct Stack<T: Default> {
        data: Box<[T]>,
        length: usize,
    }

    impl<T: Default> Stack<T> {
        pub fn new() -> Stack<T> {
            let data: Box<[T]> = Box::new([]);
            let length = 0;
            Stack { data, length }
        }

        pub fn is_empty(&self) -> bool {
            self.length == 0
        }

        pub fn size(&self) -> usize {
            self.length
        }

        pub fn peek(&self) -> Option<&T> {
            match &self.is_empty() {
                true => None,
                false => Some(&self.data[self.length - 1]),
            }
        }

        pub fn pop(&mut self) -> Option<&T> {
            if self.length == 0_usize {
                return None;
            }

            self.length = self.length - 1_usize;
            Some(&self.data[self.length])
        }

        pub fn push(&mut self, item: T) {
            if self.length == self.data.len() {
                let new_length = (self.length * 3_usize / 2_usize) + 1_usize;

                // Use mem::replace to avoid having to clone the original values
                // Replace the original data with placeholder (empty slice)
                // But save the original
                // Convert the original to a vec, resize it with the T's default value
                // Then convert to a slice and swap it back in
                // Even though this uses Vec, which is a stack datastructure already,
                // it really is only using it for runtime array allocation, so I
                // don't think it counts as cheating
                let data = std::mem::replace(&mut self.data, Box::new([]));
                let mut data = data.into_vec();
                data.resize_with(new_length, T::default);
                std::mem::replace(&mut self.data, data.into_boxed_slice());
            }

            self.data[self.length] = item;
            self.length += 1_usize;
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_push_pop() {
            let mut stack = Stack::new();
            stack.push(0);
            stack.push(123);
            stack.push(456);
            assert_eq!(stack.pop(), Some(&456));
            assert_eq!(stack.pop(), Some(&123));
            assert_eq!(stack.pop(), Some(&0));
            assert_eq!(stack.pop(), None);

            let mut stack = Stack::new();
            stack.push("0".to_string());
            stack.push("123".to_string());
            stack.push("456".to_string());
            assert_eq!(stack.pop(), Some(&"456".to_string()));
            assert_eq!(stack.pop(), Some(&"123".to_string()));
            assert_eq!(stack.pop(), Some(&"0".to_string()));
            assert_eq!(stack.pop(), None);
        }

        #[test]
        fn test_peek_size_is_empty() {
            let mut stack = Stack::new();
            assert_eq!(stack.is_empty(), true);
            assert_eq!(stack.peek(), None);
            assert_eq!(stack.size(), 0);

            stack.push(42);
            assert_eq!(stack.is_empty(), false);
            assert_eq!(stack.peek(), Some(&42));
            assert_eq!(stack.size(), 1);

            stack.push(-999);
            assert_eq!(stack.is_empty(), false);
            assert_eq!(stack.peek(), Some(&-999));
            assert_eq!(stack.size(), 2);

            stack.pop();
            stack.pop();

            assert_eq!(stack.is_empty(), true);
            assert_eq!(stack.peek(), None);
            assert_eq!(stack.size(), 0);
        }
    }
}
