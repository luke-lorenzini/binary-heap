use no_std_binary_heap::MaxHeap;

#[test]
fn test_new_heap_empty() {
    let my_heap: MaxHeap<char> = MaxHeap::new();
    assert!(my_heap.is_empty());
}

#[test]
fn test_new_heap_len_zero() {
    let my_heap: MaxHeap<char> = MaxHeap::new();
    assert_eq!(0, my_heap.len());
}

#[test]
fn test_heap_push() {
    let mut my_heap = MaxHeap::new();
    assert_eq!(None, my_heap.peek());
    my_heap.push(3);

    assert_eq!(1, my_heap.len());
    assert_eq!(Some(&3), my_heap.peek());
}

#[test]
fn test_heap_multi_push() {
    let mut my_heap = MaxHeap::new();
    my_heap.push(-3);
    my_heap.push(17);

    assert_eq!(2, my_heap.len());
    assert_eq!(Some(&17), my_heap.peek());
}

#[test]
fn test_pop_single() {
    let val = 17;
    let mut my_heap = MaxHeap::new();
    my_heap.push(val);

    assert_eq!(Some(val), my_heap.pop());
}

#[test]
fn test_pop_multi() {
    let val1 = 17;
    let val2 = -5;
    let val3 = 100;
    let mut my_heap = MaxHeap::new();
    my_heap.push(val1);
    my_heap.push(val2);
    my_heap.push(val3);

    my_heap.pop();
    my_heap.pop();

    assert_eq!(Some(val2), my_heap.pop());
}

#[test]
fn clear_heap() {
    let val1 = 17;
    let val2 = -5;
    let val3 = 100;
    let mut my_heap = MaxHeap::new();
    my_heap.push(val1);
    my_heap.push(val2);
    my_heap.push(val3);

    my_heap.clear();

    assert!(my_heap.is_empty());
}
