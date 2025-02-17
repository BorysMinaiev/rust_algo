use crate::collections::indexed_heap::IndexedHeap;
use crate::collections::min_max::MinimMaxim;
use crate::graph::edges::weighted_edge_trait::WeightedEdgeTrait;
use crate::graph::graph::Graph;
use crate::numbers::num_traits::add_sub::Addable;
use crate::numbers::num_traits::zero_one::ZeroOne;

pub trait Distances<W: Addable + PartialOrd + Copy + ZeroOne> {
    fn distances_from(&self, source: usize) -> Vec<Option<(W, usize, usize)>>;

    fn distance(&self, source: usize, mut destination: usize) -> Option<(W, Vec<(usize, usize)>)> {
        let dist = self.distances_from(source);
        match dist[destination] {
            None => None,
            Some((w, ..)) => {
                let mut path = Vec::new();
                while destination != source {
                    let (_, from, edge) = dist[destination].unwrap();
                    path.push((from, edge));
                    destination = from;
                }
                path.reverse();
                Some((w, path))
            }
        }
    }
}

impl<W: Addable + PartialOrd + Copy + ZeroOne, E: WeightedEdgeTrait<W>> Distances<W> for Graph<E> {
    fn distances_from(&self, source: usize) -> Vec<Option<(W, usize, usize)>> {
        let n = self.vertex_count();
        let mut res = vec![None; n];
        let mut heap = IndexedHeap::new(n);
        heap.add_or_adjust(source, (W::zero(), source, self[source].len()));
        while let Some((cur, dist)) = heap.pop() {
            res[cur] = Some(dist);
            let dist = dist.0;
            for (i, e) in self[cur].iter().enumerate() {
                let next = e.to();
                let total = dist + e.weight();
                let was = heap.value(next);
                let mut next_dist = (total, cur, i);
                if let Some(was) = was {
                    next_dist.minim(*was);
                }
                heap.add_or_adjust(next, next_dist);
            }
        }
        res
    }
}
