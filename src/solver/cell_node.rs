use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CellNode {
    pub index: usize,
    pub popcount: usize,
}

impl CellNode {
    pub fn new(index: usize, popcount: usize) -> Self {
        Self { index, popcount }
    }
}

impl PartialOrd for CellNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CellNode {
    fn cmp(&self, other: &Self) -> Ordering {
        // flip the order to make it a min-heap
        other.popcount.cmp(&self.popcount)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::collections::BinaryHeap;

    #[test]
    fn test_cell_node_ordering() {
        let mut heap = BinaryHeap::new();

        heap.push(CellNode::new(0, 3));
        heap.push(CellNode::new(1, 1));
        heap.push(CellNode::new(2, 2));

        assert_eq!(heap.pop().unwrap().index, 1);
        assert_eq!(heap.pop().unwrap().index, 2);
        assert_eq!(heap.pop().unwrap().index, 0);
    }
}
