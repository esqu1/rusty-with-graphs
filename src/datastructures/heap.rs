struct BinaryHeap<V: Ord> {
    heap_array: Vec<V>,
}

pub struct MaxHeap<V: Ord> {
    heap: BinaryHeap<V>,
}

pub struct MinHeap<V: Ord> {
    heap: BinaryHeap<V>,
}

impl<'a, V: Ord> BinaryHeap<V> {
    fn size(self: &Self) -> usize {
        self.heap_array.len()
    }

    fn new(arr: Vec<V>) -> BinaryHeap<V> {
        BinaryHeap { heap_array: arr }
    }

    fn iter(&'a self) -> std::slice::Iter<'a, V> {
        (&self.heap_array).iter()
    }
}

impl<V: Ord> MaxHeap<V> {
    fn compare(&self, a: usize, b: usize) -> bool {
        let arr = &self.heap.heap_array;
        arr.get(a)
            .and_then(|parent_value| {
                arr.get(b)
                    .and_then(|child_value| Some(parent_value >= child_value))
            })
            .unwrap_or_default()
    }
}

impl<V: Ord> MinHeap<V> {
    fn compare(&self, a: usize, b: usize) -> bool {
        let arr = &self.heap.heap_array;
        arr.get(a)
            .and_then(|parent_value| {
                arr.get(b)
                    .and_then(|child_value| Some(parent_value <= child_value))
            })
            .unwrap_or_default()
    }
}

macro_rules! impl_heaps {
    (for $($t:ident<$t1:ident>), +) => {
        $(impl<$t1: Ord> $t<$t1> {
            fn heapify_helper(&mut self, n: usize, index: usize) {
                let left = 2 * index + 1;
                let right = 2 * index + 2;
                let mut max_index = index;

                // find which vertex should become parent among index and children
                if left < n && !self.compare(max_index, left) {
                    max_index = left;
                }
                if right < n && !self.compare(max_index, right) {
                    max_index = right;
                }

                // if we need to swap, recurse
                if max_index != index {
                    self.heap.heap_array.swap(max_index, index);
                    self.heapify_helper(n, max_index);
                }
            }
            fn heapify(&mut self) {
                let n = self.size();
                for i in (0..(n / 2 - 1)).rev() {
                    self.heapify_helper(n, i);
                }
            }

            pub fn size(&self) -> usize {
                self.heap.size()
            }

            pub fn insert(self: &mut Self, value: V) {
                {
                    let arr = &mut self.heap.heap_array;
                    arr.push(value);
                }
                let mut i = self.heap.heap_array.len();
                while true {
                    if i == 0 {
                        break;
                    }
                    let parent : usize = (i-1) / 2;
                    if !self.compare(parent, i) {
                        let arr = &mut self.heap.heap_array;
                        arr.swap(parent, i);
                    }
                    i = parent;
                }
            }

            pub fn peek(self: &Self) -> Option<&V>{
                self.heap.heap_array.get(0)
            }

            pub fn pop(self: &mut Self) -> Option<V> {
                if self.size() == 0 {
                    None
                } else {
                    let n = self.size();
                    let arr = &mut self.heap.heap_array;
                    arr.swap(0, n - 1);
                    let x = arr.pop();
                    self.heapify();
                    x
                }
            }
        }
        impl<$t1: Ord> From<Vec<$t1>> for $t<$t1> {
            fn from(arr: Vec<V>) -> Self {
                let mut heap = Self {
                    heap: BinaryHeap::new(arr), 
                };
                heap.heapify();
                heap
            }
        }
    )*
    }
}

impl_heaps!(for MaxHeap<V>, MinHeap<V>);
