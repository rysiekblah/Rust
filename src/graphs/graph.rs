
use std::fmt;
use graphs::adj_mx;
use graphs::adj_mx::Mx;
use std::fmt::{Formatter, Error};

#[derive(Debug)]
struct UndirectedGraph {
    _vertices: usize,
    _edges: usize,
    _adj: Mx
}

impl UndirectedGraph  {

    pub fn new(v: usize) -> UndirectedGraph {
        UndirectedGraph {
            _vertices: v,
            _edges: 0,
            _adj: adj_mx::new(v)
        }
    }

    pub fn add_edge(&mut self, v: usize, w: usize) {
        adj_mx::add_adj_item(&mut self._adj, v,w);
        adj_mx::add_adj_item(&mut self._adj, w, v);
        self._edges = self._edges + 1;
    }

    pub fn adj(&self, v: usize) -> &Vec<usize> {
        adj_mx::adj(&self._adj, v)
    }

    pub fn degree(self, v: usize) -> usize {
        self.adj(v).len()
    }

}

impl fmt::Display for UndirectedGraph {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "UndirectedGraph (vert={}, edges={})\n", self._vertices, self._edges)?;
        write!(f, "{}", self._adj)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn create_graph() {
        let mut gr = UndirectedGraph::new(13);
        gr.add_edge(0, 5);
        gr.add_edge(4, 3);
        gr.add_edge(0, 1);
        gr.add_edge(9, 12);
        gr.add_edge(6, 4);
        gr.add_edge(5, 4);
        gr.add_edge(0, 2);
        gr.add_edge(11, 12);
        gr.add_edge(9, 10);
        gr.add_edge(0, 6);
        gr.add_edge(7, 8);
        gr.add_edge(9, 11);
        gr.add_edge(5, 3);

        println!("{}", gr);

    }

}