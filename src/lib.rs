pub mod data;


pub mod linked_list {

    #[derive(Debug, PartialEq)]
    pub struct LinkedList<T> {
        head: Option<Box<Node<T>>>,
    }

    #[derive(Debug, PartialEq)]
    pub struct Node<T> {
        value: T,
        next: Option<Box<Node<T>>>,
    }

    impl<T> LinkedList<T> {
        pub fn new() -> LinkedList<T> {
            LinkedList { head: None }
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
}

pub mod old_graph {
    use std::collections::HashSet;

    type NodeIndex = usize;

    #[derive(Debug)]
    pub struct Graph<T>
    where
        T: std::fmt::Debug,
    {
        nodes: Vec<NodeData<T>>,
        edges: Vec<Vec<EdgeData>>,
        reversed_edges: Vec<Vec<EdgeData>>,
    }

    #[derive(Debug)]
    struct NodeData<T> {
        data: T,
    }

    #[derive(Debug)]
    struct EdgeData {
        target: NodeIndex,
    }

    impl<T> Graph<T>
    where
        T: std::fmt::Debug,
    {
        pub fn new() -> Graph<T> {
            Graph {
                nodes: Vec::new(),
                edges: Vec::new(),
                reversed_edges: Vec::new(),
            }
        }

        pub fn add_node(&mut self, data: T) -> NodeIndex {
            let index = self.nodes.len();
            self.nodes.push(NodeData { data });
            self.edges.push(Vec::new());
            self.reversed_edges.push(Vec::new());
            index
        }

        pub fn add_edge(&mut self, source: NodeIndex, target: NodeIndex) -> Result<(), ()> {
            // should check duplicates?

            match self.edges.get_mut(source) {
                Some(edge) => edge.push(EdgeData { target }),
                None => {
                    return Err(());
                }
            };

            match self.reversed_edges.get_mut(target) {
                Some(edge) => {
                    edge.push(EdgeData { target: source });
                    Ok(())
                }
                None => Err(()),
            }
        }

        pub fn is_tree(&self) -> bool {
            // Empty graph is not a tree
            if self.nodes.len() == 0 {
                return false;
            }
            // Must have n - 1 edges and be connected
            (self.num_edges() == self.nodes.len() - 1) && self.is_connected()
        }

        fn num_edges(&self) -> usize {
            self.edges.iter().map(|e| e.len()).sum()
        }

        fn is_connected(&self) -> bool {
            let head = match self.to_head() {
                Some(node_index) => node_index,
                None => {
                    return false;
                }
            };

            let mut visited = HashSet::new();

            let mut target_node = 0;
            visited.insert(target_node);

            loop {
                let current_edge = self.edges.get(target_node).unwrap();

                match current_edge.get(0) {
                    Some(edge) => {
                        target_node = edge.target;
                        // Cycles means the graph is not a tree
                        if visited.contains(&target_node) {
                            return false;
                        }

                        visited.insert(target_node);
                    }
                    None => {
                        return visited.len() == self.nodes.len();
                    }
                };
            }
        }

        fn to_head(&self) -> Option<NodeIndex> {
            let mut visited = HashSet::new();
            let edges = &self.reversed_edges;
            // Start with the first node
            let mut target_node = 0;
            visited.insert(target_node);

            loop {
                // Unwrap is safe because we never add a node without
                // also adding an empty vector of edges here.
                let current_edges = edges.get(target_node).unwrap();

                match current_edges.get(0) {
                    Some(edge) => {
                        target_node = edge.target;
                        // Cycles means the graph is not a tree
                        if visited.contains(&target_node) {
                            return None;
                        }

                        visited.insert(target_node);
                    }
                    None => {
                        return Some(target_node);
                    }
                };
            }
        }
    }

    #[cfg(test)]
    mod tests {

        use super::*;

        #[test]
        fn test_is_tree() {
            let mut graph = Graph::new();
            let a = graph.add_node(1);
            let b = graph.add_node(1);
            // Not a tree because num edges != num nodes - 1
            assert!(!graph.is_tree());

            graph.add_edge(a, b);
            assert!(graph.is_tree());

            let c = graph.add_node(1);
            let d = graph.add_node(1);
            graph.add_edge(c, d);
            graph.add_edge(d, c);

            // num edges == num nodes - 1
            // but not a tree because it's not connected
            assert!(!graph.is_tree());
        }

    }
}

pub mod primes {

    use std::iter;

    pub fn trial_division(n: usize) -> usize {
        let mut primes = Vec::new();
        for i in 2..n {
            if !primes.iter().any(|&p| i % p == 0) {
                primes.push(i);
            }
        }
        primes.len()
    }

    pub fn sieve_of_eratosthenes_odds(n: usize) -> usize {
        if n <= 2 {
            return 0;
        }

        let mut values: Vec<bool> = (3..n).step_by(2).map(|_i| true).collect();

        for index in 0..(values.len()) {
            let value = values[index];
            if !value {
                continue;
            }
            let p = 3 + 2 * index;
            // optimization: all composites c, p < c < p^2 are already marked
            let mut i = p * p;

            // optimization: therefore, we can break if p^2 > n
            if i > n {
                break;
            }
            while i < n {
                values[(i - 3_usize) / 2] = false;
                i += 2 * p;
            }
        }
        values.into_iter().filter(|&p| p).count() + 1
    }

    // not implemented yet
    pub fn sieve_of_eratosthenes_naive(n: usize) -> usize {
        if n <= 2 {
            return 0;
        }

        let mut values = vec![true; n];
        values[0] = false;
        values[1] = false;

        for p in 0..n {
            if !values[p] {
                continue;
            }

            // optimization: all composites c, p < c < p^2 are already marked
            let mut i = p * p;

            // optimization: therefore, we can break if p^2 > n
            if i > n {
                break;
            }
            while i < n {
                values[i] = false;
                i += p;
            }
        }
        values.into_iter().filter(|&p| p).count()
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_trial_division() {
            let values: Vec<usize> = vec![0, 1, 2, 3, 6, 100000];
            let num_primes: Vec<usize> = vec![0, 0, 0, 1, 3, 9592];
            for (value, num_prime) in values.into_iter().zip(num_primes) {
                assert_eq!(trial_division(value), num_prime);
            }
        }

        #[test]
        fn test_sieve_of_eratosthenes_naive() {
            let values: Vec<usize> = vec![0, 1, 2, 3, 6, 100000];
            let num_primes: Vec<usize> = vec![0, 0, 0, 1, 3, 9592];
            for (value, num_prime) in values.into_iter().zip(num_primes) {
                assert_eq!(sieve_of_eratosthenes_naive(value), num_prime);
            }
        }

        #[test]
        fn test_sieve_of_eratosthenes_odds() {
            let values: Vec<usize> = vec![0, 1, 2, 3, 6, 100000];
            let num_primes: Vec<usize> = vec![0, 0, 0, 1, 3, 9592];
            for (value, num_prime) in values.into_iter().zip(num_primes) {
                assert_eq!(sieve_of_eratosthenes_odds(value), num_prime);
            }
        }
    }
}
