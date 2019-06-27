pub mod data {

    // Regular ole stack, only using Vec a tiny bit
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

        const MAX_LOAD_FACTOR: f64 = 0.5;

        #[derive(Debug, Clone, PartialEq)]
        enum BucketElement<T: Clone + Hash + PartialEq> {
            NonEmpty(T),
            Empty,
            Deleted,
        }

        pub struct HashSet<T: Clone + Hash + PartialEq> {
            data: Vec<BucketElement<T>>,
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
                self.data = Vec::new();
                self.size = 0;
                self.loaded = 0;
            }

            pub fn new() -> HashSet<T> {
                HashSet {
                    data: Vec::new(),
                    size: 0,
                    loaded: 0,
                }
            }

            pub fn add(&mut self, item: T) -> bool {
                if self.loaded as f64 >= self.data.len() as f64 * MAX_LOAD_FACTOR {
                    // Add two so if this is the first entry added, we still have an open bucket
                    // because we always need slack to know if we're done looking for an item.
                    let num_buckets = self.data.len() * 2 + 2;
                    self.resize(num_buckets)
                }

                let index = self.index_of(&item);

                if let BucketElement::NonEmpty(_) = self.data[index] {
                    return false;
                }

                self.data[index] = BucketElement::NonEmpty(item);
                self.size += 1;
                self.loaded += 1;
                return true;
            }

            fn resize(&mut self, new_size: usize) {
                // Reset to bigger HashSet
                let new_data = vec![BucketElement::Empty; new_size];
                let old_data = std::mem::replace(&mut self.data, new_data);

                self.size = 0;
                self.loaded = 0;

                // Copy into the bigger HashSet
                for bucket in old_data.into_iter() {
                    if let BucketElement::NonEmpty(item) = bucket {
                        self.add(item);
                    }
                }
            }

            pub fn index_of(&self, item: &T) -> usize {
                let mut hasher = DefaultHasher::new();
                item.hash(&mut hasher);

                let hash = hasher.finish();

                let mut k = 1;
                let mut index = (hash as usize + k * k) % self.data.len();

                let found_index = |i| match &self.data[i] {
                    BucketElement::Empty => true,
                    BucketElement::NonEmpty(item2) if item2 == item => true,
                    _ => false,
                };

                while !found_index(index) {
                    k += 1;
                    index = (hash as usize + k * k) % self.data.len();
                }
                return index;
            }

            pub fn contains(&self, item: &T) -> bool {
                if self.size == 0 {
                    return false;
                }

                let index = self.index_of(&item);

                match self.data[index] {
                    BucketElement::NonEmpty(_) => true,
                    _ => false,
                }
            }

            pub fn remove(&mut self, item: &T) -> bool {
                if self.size == 0 {
                    return false;
                }

                let index = self.index_of(&item);

                match self.data[index] {
                    BucketElement::NonEmpty(_) => {
                        self.data[index] = BucketElement::Deleted;
                        self.size -= 1;
                        return true;
                    }
                    _ => false,
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

    // Way cleaner than Open Addressing - this is standard array + vector implementation
    pub mod hash_set2 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        const MAX_LOAD_FACTOR: f64 = 1.0;

        pub struct HashSet<T: Hash + Clone + PartialEq> {
            data: Vec<Vec<T>>,
            size: usize,
        }

        impl<T: Hash + Clone + PartialEq> HashSet<T> {
            pub fn new() -> HashSet<T> {
                HashSet {
                    data: Vec::new(),
                    size: 0,
                }
            }

            pub fn is_empty(&self) -> bool {
                self.size == 0
            }

            pub fn size(&self) -> usize {
                self.size
            }

            pub fn clear(&mut self) {
                self.data = Vec::new();
                self.size = 0;
            }

            pub fn contains(&self, item: &T) -> bool {
                if self.size == 0 {
                    return false;
                }
                let index = self.bucket_of(item);

                self.data[index].iter().any(|item2| item2 == item)
            }

            pub fn add(&mut self, item: T) -> bool {
                if self.contains(&item) {
                    return false;
                }

                if self.size as f64 + 1.0 >= self.data.len() as f64 * MAX_LOAD_FACTOR {
                    let new_data_length = self.size * 2 + 1;
                    self.resize(new_data_length);
                }

                let index = self.bucket_of(&item);
                self.data[index].push(item);
                self.size += 1;

                true
            }

            fn resize(&mut self, new_length: usize) {
                let new_data = vec![Vec::new(); new_length];
                let old_data = std::mem::replace(&mut self.data, new_data);

                for v in old_data.into_iter() {
                    for item in v.into_iter() {
                        self.add(item);
                    }
                }
            }

            pub fn remove(&mut self, item: &T) -> bool {
                if self.size == 0 {
                    return false;
                }
                let index = self.bucket_of(item);

                let bucket = &mut self.data[index];
                let item_index = bucket.iter().position(|item2| item2 == item);

                match item_index {
                    Some(i) => {
                        bucket.remove(i);
                        self.size -= 1;
                        return true;
                    }
                    None => false,
                }
            }

            fn bucket_of(&self, item: &T) -> usize {
                let mut hasher = DefaultHasher::new();
                item.hash(&mut hasher);
                let hash = hasher.finish();

                hash as usize % self.data.len()
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

    // Graph!
    pub mod graph {
        use std::collections::HashMap;

        type NodeLabel = usize;

        pub struct Graph<T> {
            nodes: Vec<T>,
            edges: Vec<Vec<NodeLabel>>,
        }

        impl<T> Graph<T> {

            pub fn new(nodes: Vec<T>, edge_list: Vec<(usize, usize)>) -> Graph<T> {                
                let mut edges = vec![Vec::new(); nodes.len()];
                for (head_id, tail_id) in edge_list.iter() {                    
                    edges[*head_id].push(*tail_id);
                }

                Graph {
                    nodes, 
                    edges
                }
            }

            pub fn bfs(&self, source_id: usize, destination_id: usize) -> Option<Vec<usize>> {
                let mut queue = vec!(source_id);
                let mut parents = HashMap::new();

                while let Some(node) = queue.pop() {                    

                    if node == destination_id {
                        return Some(self.backtrace(node, &parents));
                    }

                    for edge in self.edges[node].iter() {
                        if !parents.contains_key(edge) {
                            queue.push(*edge);
                            parents.insert(*edge, node);
                        }
                    }

                }
                None
            }

            fn backtrace(&self, node: usize, parents: &HashMap<usize, usize>) -> Vec<usize> {
                let mut trace = vec!(node);
                let mut current = node;
                while let Some(parent) = parents.get(&current) {
                    trace.push(*parent);
                    current = *parent;
                }
                trace.into_iter().rev().collect()
            }
        }



        #[cfg(test)]
        mod tests {
            use super::*;

            #[test]
            fn test_bfs() {
                let graph = Graph::new(vec![0; 5], vec!((0,1), (1,2), (2,4), (3, 4)));

                assert_eq!(Some(vec!(0, 1, 2, 4)), graph.bfs(0, 4));
                assert_eq!(None, graph.bfs(4, 0));

                let graph = Graph::new(vec![0; 5], vec!((0,1), (1,2), (2,3), (3, 4), (2, 4)));
                
                assert_eq!(Some(vec!(0, 1, 2, 4)), graph.bfs(0, 4));

            }
        }
    }
}
