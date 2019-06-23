pub mod data {

    pub mod stack {
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
                if self.length == 0 {
                    return None;
                }

                self.length = self.length - 1;
                Some(&self.data[self.length])
            }

            pub fn push(&mut self, item: T) {
                if self.length == self.data.len() {
                    // Plus one allows growth even if current length is 0
                    let new_length = (self.length * 3 / 2) + 1;

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
                self.length += 1;
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

    // HashSet with open addressing and quadratic probing
    pub mod hash_set {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        const max_load_factor: f64 = 0.2;

        #[derive(Debug, Clone, PartialEq)]
        enum BucketElement<T: Clone + Hash + PartialEq> {
            Has(T),
            Empty,
            Deleted,
        }

        pub struct HashSet<T: Clone + Hash + PartialEq> {
            data: Box<[BucketElement<T>]>,
            size: usize,
            loaded: usize,
        }

        impl<T: Clone + Hash + PartialEq> HashSet<T> {
            pub fn is_empty(&self) -> bool {
                self.size == 0
            }

            pub fn size(&self) -> usize {
                self.size
            }

            pub fn clear(&mut self) {
                self.data = Box::new([]);
                self.size = 0;
                self.loaded = 0;
            }

            pub fn new() -> HashSet<T> {
                HashSet {
                    data: Box::new([]),
                    size: 0,
                    loaded: 0,
                }
            }

            pub fn add(&mut self, item: T) -> bool {
                if self.loaded as f64 >= self.data.len() as f64 * max_load_factor {
                    // Add two, so even if we started at 0 and are adding 1 item now, we still have capacity.
                    let num_buckets = self.data.len() * 3 + 2;

                    // Reset to bigger HashSet
                    let new_data = vec![BucketElement::Empty; num_buckets].into_boxed_slice();
                    let old_data = std::mem::replace(&mut self.data, new_data);

                    self.size = 0;
                    self.loaded = 0;

                    // Copy into the bigger HashSet
                    for bucket in old_data.into_iter() {
                        if let BucketElement::Has(item) = bucket {
                            self.add(item.clone());
                        }
                    }
                }

                let mut hasher = DefaultHasher::new();
                item.hash(&mut hasher);
                let hash = hasher.finish();

                let mut k = 1;
                let mut index = (hash as usize + k * k) % self.data.len();

                loop {
                    match self.data[index] {
                        BucketElement::Empty => {
                            self.data[index] = BucketElement::Has(item.clone());
                            self.size += 1;
                            self.loaded += 1;
                            return true;
                        }

                        BucketElement::Has(ref item2) if item2 == &item => {
                            return false;
                        }

                        _ => (),
                    }
                    k += 1;
                    index = (hash as usize + k * k) % self.data.len();
                }
            }

            pub fn contains(&self, item: &T) -> bool {
                if self.size == 0 {
                    return false;
                }
                let mut hasher = DefaultHasher::new();
                item.hash(&mut hasher);
                let hash = hasher.finish();

                let mut k = 1;
                let mut index = (hash as usize + k * k) % self.data.len();

                loop {
                    match self.data[index] {
                        BucketElement::Empty => {
                            return false;
                        }

                        BucketElement::Has(ref item2) if item2 == item => {
                            return true;
                        }

                        _ => (),
                    }
                    k += 1;
                    index = (hash as usize + k * k) % self.data.len();
                }
            }

            pub fn remove(&mut self, item: &T) -> bool {
                let mut hasher = DefaultHasher::new();
                item.hash(&mut hasher);
                let hash = hasher.finish();

                let mut k = 1;
                let mut index = (hash as usize + k * k) % self.data.len();

                loop {
                    match self.data[index] {
                        BucketElement::Empty => {
                            return false;
                        }

                        BucketElement::Has(ref item2) if item2 == item => {
                            self.data[index] = BucketElement::Deleted;
                            self.size -= 1;
                            return true;
                        }

                        _ => (),
                    }
                    k += 1;
                    index = (hash as usize + k * k) % self.data.len();
                }
            }
        }

        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test_add_remove() {
                let mut hash_set = HashSet::new();
                let result = hash_set.add(4);
                assert_eq!(result, true);

                let result = hash_set.add(4);
                assert_eq!(result, false);

                hash_set.remove(&4);
                let result = hash_set.add(4);
                assert_eq!(result, true);
            }

            #[test]
            fn test_is_empty() {
                let mut hash_set = HashSet::new();
                assert!(hash_set.is_empty());

                hash_set.add(21);
                assert!(!hash_set.is_empty());

                hash_set.remove(&21);
                assert!(hash_set.is_empty());
            }

            #[test]
            fn test_contains() {
                let mut hash_set = HashSet::new();
                assert_eq!(hash_set.contains(&"abc"), false);

                hash_set.add("abc");
                assert!(hash_set.contains(&"abc"));

                assert!(!hash_set.contains(&"123"));

                hash_set.remove(&"abc");
                assert!(!hash_set.contains(&"abc"));
            }

            #[test]
            fn test_clear() {
                let mut hash_set = HashSet::new();
                hash_set.add(123);

                hash_set.clear();

                assert!(hash_set.is_empty());
                assert!(!hash_set.contains(&123));
                assert_eq!(hash_set.size(), 0);
            }

            #[test]
            fn test_benchmarks_tolerable() {
                let mut hash_set = HashSet::new();

                for i in 0..1000000 {
                    hash_set.add(i);
                }

                for i in 0..1000000 {
                    if i % 2 == 0 {
                        hash_set.remove(&i);
                    }
                }
            }
        }
    }
}
