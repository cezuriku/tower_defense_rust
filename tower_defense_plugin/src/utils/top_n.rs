use std::collections::BinaryHeap;

pub struct TopN<T> {
    heap: BinaryHeap<T>,
    max_size: usize,
}

impl<T: Ord + Clone> TopN<T> {
    pub fn new(max_size: usize) -> Self {
        TopN {
            heap: BinaryHeap::with_capacity(max_size + 1),
            max_size,
        }
    }

    pub fn insert(&mut self, element: T) {
        self.heap.push(element);

        if self.heap.len() > self.max_size {
            self.heap.pop();
        }
    }

    pub fn get(&self) -> Vec<T> {
        self.heap.clone().into_vec()
    }

    pub fn get_sorted(&self) -> Vec<T> {
        self.heap.clone().into_sorted_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Debug;

    #[test]
    fn find_top_3() {
        let mut top_n = TopN::new(3);
        top_n.insert(10);
        top_n.insert(20);
        top_n.insert(5);
        top_n.insert(30);
        top_n.insert(15);

        assert_eq!(top_n.heap.into_sorted_vec(), vec![5, 10, 15])
    }

    #[test]
    fn find_top_3_elements() {
        pub struct Element {
            pub x: i32,
            pub y: i32,
        }

        impl Ord for Element {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                self.x.cmp(&other.x)
            }
        }
        impl PartialOrd for Element {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(self.cmp(other))
            }
        }
        impl Eq for Element {}
        impl PartialEq for Element {
            fn eq(&self, other: &Self) -> bool {
                self.x == other.x && self.y == other.y
            }
        }

        impl Clone for Element {
            fn clone(&self) -> Self {
                Element {
                    x: self.x,
                    y: self.y,
                }
            }
        }

        impl Debug for Element {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "Element {{ x: {}, y: {} }}", self.x, self.y)
            }
        }

        let mut top_n: TopN<Element> = TopN::new(3);
        top_n.insert(Element { x: 10, y: 1 });
        top_n.insert(Element { x: 20, y: 2 });
        top_n.insert(Element { x: 5, y: 3 });
        top_n.insert(Element { x: 30, y: 4 });
        top_n.insert(Element { x: 15, y: 5 });

        assert_eq!(
            top_n.heap.into_sorted_vec(),
            vec![
                Element { x: 5, y: 3 },
                Element { x: 10, y: 1 },
                Element { x: 15, y: 5 }
            ]
        )
    }
}
