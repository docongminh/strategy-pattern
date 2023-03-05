pub trait SortAlgorithm {
    fn sort<T>(&self, arr: &mut [T]);
}
// strategy merge sort
pub struct MergeSort;
impl SortAlgorithm for MergeSort {
    fn sort<T>(&self, arr: &mut [T]) {
        println!("merge sort ! #TODO")
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct SelectionSort;

impl SortAlgorithm for SelectionSort {
    fn sort<T: Ord>(&self, arr: &mut [T]) {
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
trait QuickSortUtil {
  fn quick_sort<T: Ord>(self, arr: &mut [T], low: isize, high: isize);
  fn partition<T: Ord>(self, arr: &mut [T], low: isize, high: isize) -> isize;
}

impl QuickSortUtil for QuickSort{
  fn quick_sort<T: Ord>(self, arr: &mut [T], low: isize, high: isize) {
    if low < high {
        let p = self.partition(arr, low, high);
        self.quick_sort(arr, low, p - 1);
        self.quick_sort(arr, p + 1, high);
    }
}
fn partition<T: Ord>(self, arr: &mut [T], low: isize, high: isize) -> isize {
    let pivot = high as usize;
    let mut store_index = low - 1;
    let mut last_index = high;

    loop {
        store_index += 1;
        while arr[store_index as usize] < arr[pivot] {
            store_index += 1;
        }
        last_index -= 1;
        while last_index >= 0 && arr[last_index as usize] > arr[pivot] {
            last_index -= 1;
        }
        if store_index >= last_index {
            break;
        } else {
            arr.swap(store_index as usize, last_index as usize);
        }
    }
    arr.swap(store_index as usize, pivot as usize);
    store_index
  }
}

impl SortAlgorithm for QuickSort {
    fn sort<T>(&self, arr: &mut [T]) {
        let len = arr.len();
        self.quick_sort(arr, 0, (len - 1) as isize);
    }
}

// strategy insertion sort
pub struct InsertionSort;
impl SortAlgorithm for InsertionSort {
    fn sort<T: Ord>(&self, arr: &mut [T]) {
        for i in 1..arr.len() {
            let mut j = i;
            while j > 0 && arr[j] < arr[j - 1] {
                arr.swap(j, j - 1);
                j = j - 1;
            }
        }
    }
}

pub struct BubbleSort;
impl SortAlgorithm for BubbleSort {
    fn sort<T: Ord>(&self, arr: &mut [T]) {
        for i in 0..arr.len() {
            for j in 0..arr.len() - 1 - i {
                if arr[j] > arr[j + 1] {
                    arr.swap(j, j + 1);
                }
            }
        }
    }
}

// strategy Heap Sort
pub struct HeapSort;
impl SortAlgorithm for HeapSort {
    fn sort<T: Ord>(&self, arr: &mut [T]) {
        if arr.len() <= 1 {
            return;
        }

        let last_parent = (arr.len() - 2) / 2;
        for i in (0..=last_parent).rev() {
            move_down(arr, i);
        }

        for end in (1..arr.len()).rev() {
            arr.swap(0, end);
            move_down(&mut arr[..end], 0);
        }
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

fn move_down<T: Ord>(arr: &mut [T], mut root: usize) {
    let last = arr.len() - 1;
    loop {
        let left = 2 * root + 1;
        if left > last {
            break;
        }
        let right = left + 1;
        let child = if right <= last && arr[right] > arr[left] {
            right
        } else {
            left
        };

        if arr[child] > arr[root] {
            arr.swap(root, child);
        }
        root = child;
    }
}
