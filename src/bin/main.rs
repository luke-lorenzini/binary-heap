fn main() {
    let mut my_heap = no_std_binary_heap::MaxHeap::new();
    assert_eq!(0, my_heap.len());
    assert!(my_heap.is_empty());

    my_heap.push(4);
    my_heap.push(17);
    my_heap.push(14);
    my_heap.push(1);

    assert_eq!(4, my_heap.len());
    if let Some(v) = my_heap.peek() {
        assert_eq!(17, *v);
    }
    assert_eq!(4, my_heap.len());

    my_heap.push(45);
    my_heap.push(20);
    my_heap.push(14);
    my_heap.push(12);
    my_heap.push(-31);
    my_heap.push(7);
    my_heap.push(11);
    my_heap.pop();
    my_heap.push(13);
    my_heap.push(7);

    my_heap.push(100);
    // assert_eq!(10, my_heap.len());
    // if let Some(v) = my_heap.peek() {
    //     assert_eq!(31, *v);
    // }

    while !my_heap.is_empty() {
        if let Some(v) = my_heap.pop() {
            println!("{}", v);
        }
    }
}
