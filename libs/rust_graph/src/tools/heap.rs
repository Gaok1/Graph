pub struct HeapMin<T> 
where T: Ord + Clone {
    heap: Vec<T>,
}

#[allow(unused)]
impl<T> HeapMin<T>
where T: Ord + Clone 
{
    /// Creates a new empty heap
    pub fn new() -> Self {
        HeapMin {
            heap: Vec::new(),
        }
    }

    /// Creates a new heap with a given capacity
    pub fn with_capacity(size: usize) -> Self {
        HeapMin {
            heap: Vec::with_capacity(size),
        }
    }

    /// Returns the number of elements in the heap
    pub fn len(&self) -> usize {
        self.heap.len()
    }

    /// Get the parent index and its value
    fn get_parent(&self, idx: usize) -> Option<(usize, &T)> {
        if idx == 0 {
            None
        } else {
            let parent_idx = (idx - 1) / 2;
            Some((parent_idx, &self.heap[parent_idx]))
        }
    }

    /// Get the children indices and values
    fn get_children(&self, idx: usize) -> (Option<(usize, &T)>, Option<(usize, &T)>) {
        let left_idx = 2 * idx + 1;
        let right_idx = 2 * idx + 2;

        let left_child = self.heap.get(left_idx).map(|val| (left_idx, val));
        let right_child = self.heap.get(right_idx).map(|val| (right_idx, val));

        (left_child, right_child)
    }

    /// Sift up to maintain the heap property
    fn sift_up(&mut self, idx: usize) {
        if idx == 0 {
            return;
        }
        let (parent_idx, parent_value) = self.get_parent(idx).unwrap();
        if self.heap[idx] < *parent_value {
            self.heap.swap(idx, parent_idx);
            self.sift_up(parent_idx);
        }
    }

    /// Sift down to maintain the heap property
    fn sift_down(&mut self, idx: usize) {
        let (left_child, right_child) = self.get_children(idx);

        let mut smallest_idx = idx;

        if let Some((left_idx, left_value)) = left_child {
            if *left_value < self.heap[smallest_idx] {
                smallest_idx = left_idx;
            }
        }

        if let Some((right_idx, right_value)) = right_child {
            if *right_value < self.heap[smallest_idx] {
                smallest_idx = right_idx;
            }
        }

        if smallest_idx != idx {
            self.heap.swap(idx, smallest_idx);
            self.sift_down(smallest_idx);
        }
    }

    /// Insert element into the heap
    pub fn insert(&mut self, element: T) {
        self.heap.push(element);
        self.sift_up(self.heap.len() - 1);
    }

    /// Remove the root element (the smallest element) from the heap
    pub fn pop(&mut self) -> Option<T> {
        if self.heap.is_empty() {
            return None;
        }
        let last_idx = self.heap.len() - 1;
        self.heap.swap(0, last_idx);
        let min_element = self.heap.pop();
        if !self.heap.is_empty() {
            self.sift_down(0);
        }
        min_element
    }

    /// Peek at the root element (the smallest element) without removing it
    pub fn peek(&self) -> Option<&T> {
        self.heap.get(0)
    }
}
