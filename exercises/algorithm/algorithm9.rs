/*
	heap
	This question requires you to implement a binary heap function
*/
use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self { //这里可以根据函数指针的类型，来决定是大顶堆还是小顶堆
        Self {
            count: 0,
            items: vec![T::default()],  //这里让索引从1开始更方便计算父节点和子节点的索引，0作为哨兵
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        //TODO
        self.items.push(value);
        self.count += 1;
        let mut idx = self.count;

        while idx > 1 {
            let parent_idx = self.parent_idx(idx);

            if (self.comparator)(&self.items[idx], &self.items[parent_idx]) {
                self.items.swap(idx, parent_idx);
                idx = parent_idx;
            }
            else {
                break;
            }
        }

    }

    fn parent_idx(&self, idx: usize) -> usize { //获得父节点索引
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool { //判断是否有孩子节点,这里以左孩子为准
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize { //获得左孩子节点索引
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize { //获得右孩子节点索引
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {  //这里没有判断是否有左子节点，因为这个方法是在确定有子节点的情况下调用的
        //TODO
        let left_idx = self.left_child_idx(idx);
        let right_idx = self.right_child_idx(idx);
        
        if right_idx <= self.count {
            if (self.comparator)(&self.items[left_idx], &self.items[right_idx]) {
                left_idx
            } else {
                right_idx
            }
        } else {
            left_idx
        }
		
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        //TODO
        if self.is_empty() {
            return None;
        }
        let result = self.items.swap_remove(1); //移除堆顶元素并让最后一个元素到堆顶
        self.count -= 1;
        let mut idx = 1;
        while self.children_present(idx) {
            let child_idx = self.smallest_child_idx(idx);
            if (self.comparator)(&self.items[child_idx], &self.items[idx]) {
                self.items.swap(idx, child_idx);
                idx = child_idx;
            } else {
                break;
            }
        }

        Some(result)
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}