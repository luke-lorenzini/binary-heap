//! # No Std Binary Heap
//!
//! 'no_std_binary_heap' implements a binary heap without using the standard lib.
//!
//!

#![no_std]

const ARRAY_SIZE: usize = 100;

/// Structure for holding n Option<T> values
pub struct MaxHeap<T> {
    next_position: usize,
    array: [Option<T>; ARRAY_SIZE],
}

impl<T: Copy + PartialOrd> Default for MaxHeap<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Copy + PartialOrd> MaxHeap<T> {
    /// Create a new binary heap
    ///
    /// # Examples
    ///
    /// ```
    /// use no_std_binary_heap::MaxHeap;
    /// let mut my_heap: MaxHeap<i32> = MaxHeap::new();
    /// assert_eq!(0, my_heap.len());
    /// ```
    pub fn new() -> MaxHeap<T> {
        MaxHeap {
            next_position: 0,
            array: [None; ARRAY_SIZE],
        }
    }

    /// Add an item of type T to the heap.
    ///
    /// # Examples
    ///
    /// ```
    /// use no_std_binary_heap::MaxHeap;
    /// let val1 = -3;
    /// let val2 = 17;
    /// let mut my_heap = MaxHeap::new();
    /// my_heap.push(val1);
    /// my_heap.push(val2);
    ///
    /// assert_eq!(2, my_heap.len());
    /// if let Some(v) = my_heap.peek() {
    ///        assert_eq!(val2, *v);
    /// } else {
    ///        panic!();
    /// }
    /// ```
    pub fn push(&mut self, item: T) {
        self.array[self.next_position] = Some(item);
        self.swim_up();
    }

    /// Remove the item from the top of the heap.
    ///
    /// # Examples
    ///
    /// ```
    /// use no_std_binary_heap::MaxHeap;
    /// let val1 = 17;
    /// let val2 = -5;
    /// let val3 = 100;
    /// let mut my_heap = MaxHeap::new();
    /// my_heap.push(val1);
    /// my_heap.push(val2);
    /// my_heap.push(val3);
    ///
    /// my_heap.pop();
    /// my_heap.pop();
    /// assert_eq!(Some(val2), my_heap.pop());
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        let root = self.array[0];
        self.swim_down();
        root
    }

    pub fn peek(&self) -> Option<&T> {
        self.array[0].as_ref()
    }

    /// Check if the heap is empty
    ///
    /// # Examples
    ///
    /// ```
    /// use no_std_binary_heap::MaxHeap;
    /// let mut my_heap = MaxHeap::new();
    /// my_heap.push(17);
    /// my_heap.push(-3);
    /// assert!(!my_heap.is_empty());
    /// ```
    pub fn is_empty(&self) -> bool {
        self.next_position == 0
    }

    /// Return the length of the heap.
    ///
    /// # Examples
    ///
    /// ```
    /// use no_std_binary_heap::MaxHeap;
    /// let mut my_heap = MaxHeap::new();
    /// my_heap.push(17);
    /// my_heap.push(-3);
    /// assert_eq!(2, my_heap.len())
    /// ```
    pub fn len(&self) -> usize {
        self.next_position
    }

    pub fn clear(&mut self) {
        self.next_position = 0;
    }

    fn swim_down(&mut self) {
        self.next_position -= 1;

        self.array[0] = self.array[self.next_position];

        let mut parent = 0;
        let mut left_child = self.get_left_child(&parent);
        let mut right_child = self.get_right_child(&parent);
        while left_child < self.next_position {
            if self.array[left_child] > self.array[right_child] {
                if self.array[parent] < self.array[left_child] {
                    self.array.swap(parent, left_child);
                }
                parent = left_child;
            } else {
                if self.array[parent] < self.array[right_child] {
                    self.array.swap(parent, right_child);
                }
                parent = right_child;
            }
            left_child = self.get_left_child(&parent);
            right_child = self.get_right_child(&parent);
        }
    }

    fn swim_up(&mut self) {
        if self.next_position != 0 {
            let mut child = self.next_position;
            let mut parent = self.get_parent(&child);
            while child != 0 {
                if self.array[parent] < self.array[child] {
                    self.array.swap(parent, child);
                }
                let temp = child;
                child = parent;
                parent = self.get_parent(&temp);
            }
        }
        self.next_position += 1;
    }

    fn get_parent(&self, item: &usize) -> usize {
        (item - 1) / 2
    }

    fn get_child(&self, item: &usize) -> usize {
        item * 2
    }

    fn get_left_child(&self, item: &usize) -> usize {
        self.get_child(item) + 1
    }

    fn get_right_child(&self, item: &usize) -> usize {
        self.get_child(item) + 2
    }
}
