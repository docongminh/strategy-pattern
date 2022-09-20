pub trait SortAlgorithm {
  fn sort(&self);
}

// strategy merge sort
pub struct MergeSort;
impl SortAlgorithm for MergeSort {
  fn sort(&self) {
    println!("merge sort ! #TODO")
  }
}

// strategy selection sort
pub struct SelectionSort;

impl SortAlgorithm for SelectionSort {
  fn sort(&self) {
    println!("selection sort ! #TODO");
  }
}

// strategy quick sort
pub struct QuickSort;

impl SortAlgorithm for QuickSort {
  fn sort(&self) {
    println!("Quick sort ! # TODO");
  }
}

// strategy insertion sort
pub struct InsertionSort;
impl SortAlgorithm for InsertionSort {
  fn sort(&self) {
    println!("Insertion sort ! TODO");
  }
}

// strategy bubble sort
pub struct BubbleSort;
impl SortAlgorithm for BubbleSort {
  fn sort(&self) {
    println!("bubble sort ! #TODO");
  }
}

// strategy Heap Sort
pub struct HeapSort;
impl SortAlgorithm for HeapSort {
  fn sort(&self) {
    println!("Heap sort ! #TODO");
  }
}

// strategy Counting sort
pub struct CountingSort;
impl SortAlgorithm for CountingSort {
  fn sort(&self) {
    println!("Counting sort ! #TODO");
  }
}
