# Learning: strategy-pattern

Combined sort-algorithm with strategy pattern for learning purpose. Easy to switching between sort algorithm to test

```rs
  fn main() {
    println!("----Sort numbers----");

    let mut arr = [4, 5, 2, -31, 0, 23, 8, 7, 1];
    println!("Before: {:?}", arr);
    
    Sort::sort(SelectionSort, &mut arr);
    println!("After:  {:?}\n", arr);

    println!("----Sort strings----");
    
    let mut strings = ["money", "crypto", "stock", "car", "house", "gold"];
    println!("Before: {:?}", strings);
    
    Sort::sort(SelectionSort, &mut strings);
    println!("After:  {:?}\n", strings);
}
```

# TODO
- [ ] Add measure performance (show run time)
- [ ] Add cli
- [ ] Impl count sort
- [ ] Impl merge sort
