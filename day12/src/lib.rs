use std::collections::HashMap;

use std::fmt;
use std::hash::Hash;

// Sorted based on key, ordering of pairs with matching keys is undefined.

pub struct MinHeapKeyValue<K, V> where
  K : PartialOrd,
  V : PartialOrd {
  // Stores the actual values
  heap : Vec<(K, V)>,
  // Stores a reverse mapping from the value (which
  // must be unique) to the index in the heap.
  reverse_mapping : HashMap<V, usize>,
}

impl<K, V> MinHeapKeyValue<K, V> where 
  K : fmt::Debug + PartialOrd,
  V : fmt::Debug + Copy + PartialOrd + Hash + Eq, {

  fn left_child(idx : usize) -> usize {
    idx * 2 + 1
  }
  fn right_child(idx : usize) -> usize {
    idx * 2 + 2
  }
  fn parent(idx : usize) -> usize {
    (idx - 1) / 2
  }

  /// Performs a bubble up on the element at the given index,
  /// and returns the index that element ends up at.
  /// Assumes that 
  fn bubble_up(&mut self, idx : usize) {
    let mut curr = idx;
    while curr > 0 {
      let p = Self::parent(curr);
      if self.heap[curr] >= self.heap[p] {
        break;
      }
      self.reverse_mapping.insert(self.heap[p].1, curr);
      self.heap.swap(curr, p);
      curr = p;
    }
    self.reverse_mapping.insert(self.heap[curr].1, curr);
  }

  fn swap(&mut self, a : usize, b : usize) {
    // Swap elements in the heap, but also update our
    // reverse mapping.
    let aVal = self.heap[a].1;
    let bVal = self.heap[b].1;
    self.heap.swap(a, b);
    self.reverse_mapping.insert(aVal, b);
    self.reverse_mapping.insert(bVal, a);
  }
  
  /// Ensure the tree starting at root idx meets our invariants.
  /// It is assumed that the children of idx form trees that
  /// meet the invariants however.
  fn min_heapify(&mut self, idx : usize) {
    let l = Self::left_child(idx);
    let r = Self::right_child(idx);
    let mut s = idx;
    if l < self.heap.len() && self.heap[l] < self.heap[s] {
      s = l;
    }
    if r < self.heap.len() && self.heap[r] < self.heap[s] {
      s = r;
    }
    // If the smallest value is not the parent but one of the children,
    // swap parent and child and recurse.
    // and re
    if s != idx {
      self.swap(idx, s);
      self.min_heapify(s);
    }
  }

  fn new() -> Self {
    Self{
      heap : Vec::new(),
      reverse_mapping : HashMap::new()
    }
  }

  fn insert(&mut self, key : K, value : V) {
    // Insert at back of heap.
    self.heap.push((key, value));

    // Bubble up will also insert the reverse mapping we need
    self.bubble_up(self.heap.len() - 1);
  }

  /// Take the minimum element off the heap and return it.
  /// Returns None if the heap is empty.
  fn pop(&mut self) -> Option<(K, V)> {
    if self.heap.is_empty() {
      return None
    }
    if self.heap.len() == 1 {
      self.reverse_mapping.clear();
      return self.heap.pop();
    }

    // Swap first element to the back
    let last_idx = self.heap.len() - 1;
    self.heap.swap(0, last_idx);

    // Take the result
    let res = self.heap.pop().unwrap();
    let resVal = &res.1;

    let otherVal = self.heap[0].1;

    // Update the reverse lookup
    self.reverse_mapping.remove(resVal);
    self.reverse_mapping.insert(otherVal, 0);

    self.min_heapify(0);
    
    Some(res)
  }

  /// Special case to update one of the keys for heap elements to a smaller
  /// value. Panics if the key is not smaller than the existing one or the
  /// given value does not exist.
  fn decrease_key(&mut self, value : &V, new_key : K) {
    let idx = self.reverse_mapping.get(value).unwrap();
    let curr_key = &mut self.heap[*idx].0;
    assert!(*curr_key >= new_key);

    *curr_key = new_key;

    // Now bubble up from this idx
    self.bubble_up(*idx);
  }

  /// Get a reference to the key associated with the given value
  fn get_key(&self, value : &V) -> Option<&K> {
    let idx = self.reverse_mapping.get(value)?;
    let pair = self.heap.get(*idx)?;
    Some(&pair.0)
  }
}

#[cfg(test)]
mod tests {
  use super::MinHeapKeyValue;
  #[test]
  fn insert_pop_one() {
    let mut heap = MinHeapKeyValue::<i32, usize>::new();
    heap.insert(10, 0);

    assert_eq!(heap.pop(), Some((10, 0)));
    assert_eq!(heap.pop(), None);
    assert_eq!(heap.pop(), None);
  }

  #[test]
  fn insert_reverse_order() {
    let mut heap = MinHeapKeyValue::<i32, usize>::new();

    heap.insert(10, 0);
    heap.insert(9, 1);
    heap.insert(8, 2);
    heap.insert(7, 3);

    // Expect the values to come out in reverse order
    assert_eq!(heap.pop(), Some((7, 3)));
    assert_eq!(heap.pop(), Some((8, 2)));
    assert_eq!(heap.pop(), Some((9, 1)));
    assert_eq!(heap.pop(), Some((10, 0)));
    assert_eq!(heap.pop(), None);
  }

  #[test]
  fn get_key_after_insert() {
    let mut heap = MinHeapKeyValue::<i32, usize>::new();

    heap.insert(10, 0);
    assert_eq!(heap.get_key(&0), Some(&10));
    assert_eq!(heap.get_key(&1), None);
    assert_eq!(heap.get_key(&2), None);

    heap.insert(9, 1);
    assert_eq!(heap.get_key(&0), Some(&10));
    assert_eq!(heap.get_key(&1), Some(&9));
    assert_eq!(heap.get_key(&2), None);
  }

  #[test]
  fn decrease_key() {
    let mut heap = MinHeapKeyValue::<i32, usize>::new();

    heap.insert(10, 0);
    heap.insert(9, 1);
    heap.insert(8, 2);
    heap.insert(7, 3);

    // Decrease the key for value 0 and check we still get the right value with get_key and pop
    heap.decrease_key(&0, 6);
    assert_eq!(heap.get_key(&0), Some(&6));
    assert_eq!(heap.get_key(&1), Some(&9));
    assert_eq!(heap.get_key(&2), Some(&8));
    assert_eq!(heap.get_key(&3), Some(&7));

    assert_eq!(heap.pop(), Some((6, 0)));
    assert_eq!(heap.pop(), Some((7, 3)));
    assert_eq!(heap.pop(), Some((8, 2)));
    assert_eq!(heap.pop(), Some((9, 1)));
  }
}
