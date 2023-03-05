pub trait SortAlgorithm {
  fn sort<T>(&self, arr: &mut[T]);
}
// strategy merge sort
pub struct MergeSort;
impl SortAlgorithm for MergeSort {
  fn sort<T>(&self, arr: &mut[T]) {
    println!("merge sort ! #TODO")
  }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct SelectionSort;

impl SortAlgorithm for SelectionSort {
  fn sort<T: Ord>(&self, arr: &mut [T]){
    let len = arr.len();
    for left in 0..len {
        let mut smallest = left;
        for right in (left + 1)..len {
            if arr[right] < arr[smallest] {
                smallest = right;
            }
        }
        arr.swap(smallest, left);
    }
  }
}

// strategy quick sort
pub struct QuickSort;

impl SortAlgorithm for QuickSort {
  fn sort<T>(&self, arr: &mut [T]) {
    println!("Quick sort ! # TODO");
  }
}

// strategy insertion sort
pub struct InsertionSort;
impl SortAlgorithm for InsertionSort {
  fn sort<T>(&self, arr: &mut [T]) {
    println!("Insertion sort ! TODO");
  }
}

// strategy bubble sort
pub struct BubbleSort;
impl SortAlgorithm for BubbleSort {
  fn sort<T>(&self, arr: &mut [T]) {
    println!("bubble sort ! #TODO");
  }
}

// strategy Heap Sort
pub struct HeapSort;
impl SortAlgorithm for HeapSort {
  fn sort<T>(&self, arr: &mut [T]) {
    println!("Heap sort ! #TODO");
  }
}

// strategy Counting sort
pub struct CountingSort;
impl SortAlgorithm for CountingSort {
  fn sort<T>(&self, arr: &mut [T]) {
    println!("Counting sort ! #TODO");
  }
}


pub struct Sort;
impl Sort {
    pub fn sort<T: SortAlgorithm, K: Ord>(sort_alg: T, arr: &mut [K]) {
        sort_alg.sort(arr)
    }
}
