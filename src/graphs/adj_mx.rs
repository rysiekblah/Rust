
use std::fmt;
use std::fmt::{Formatter, Error};

#[derive(Debug)]
pub struct Mx {
    _arr: Vec<Row>
}

#[derive(Debug)]
pub struct Row {
    _arr: Vec<usize>
}

pub fn new(v: usize) -> Mx {
    let mut mx = Vec::with_capacity(v);
    for ix in 0..v {
        mx.insert(ix, Row{_arr:Vec::new()});
    }

    Mx {
        _arr: mx
    }
}

#[derive(Debug)]
pub struct Search {
    _marked: Vec<bool>,
    _count: usize
}

pub fn add_adj_item(mx: &mut Mx, index: usize, v: usize) -> () {
    mx._arr[index]._arr.push(v);
}

pub fn adj(mx: &Mx, index: usize) -> &Vec<usize> {
    &mx._arr[index]._arr
}

pub fn degree(mx: &Mx, index: usize) -> usize {
    adj(&mx, index).len()
}

pub fn max_degree(mx: &Mx) -> Option<usize> {
    mx._arr.iter().map(|r| r._arr.len()).max()
}

pub fn has_self_loop(mx: &Mx) -> bool {
    let mut has_loop = false;
    for index in 0..mx._arr.len() {
        if mx._arr[index]._arr.iter().filter(|&item| item == &index).count() > 0 {
            has_loop = true;
            break;
        }
    }
    has_loop
}

pub fn self_loops(mx: &Mx) -> Vec<usize> {
    let mut self_loops = Vec::new();
    for index in 0..mx._arr.len() {
        if mx._arr[index]._arr.contains(&index) {
            self_loops.push(index);
        }
    }
    self_loops
}

pub fn min_degree(mx: &Mx) -> Option<usize> {
    mx._arr.iter().map(|r| r._arr.len()).min()
}

//pub fn dfs(mx: &Mx, v: usize) -> Search {
//    let mut marked = vec![false; v];
//    let mut edge_to = vec![0; v];
//    let mut count = 0;
//
//
//
//    Search {
//        _marked: marked,
//        _count: count
//    }
//}
//
//pub fn dfs_f(mx: &Mx, marked: &mut Vec<bool>, edge_to: &mut Vec<usize>, count: usize, v: &usize) -> usize {
//    marked.insert(v.clone(), true);
//    let cnt = count + 1;
//
//
////    for &w in adj(&mx, &v).iter() {
////        if !&marked[w] {
////            &edge_to[w] = v;
////            dfs(&mx, w.clone());
////        }
////    }
//}

pub fn is_connected(dfs: &Search, mx: &Mx) -> bool {
    dfs._count == mx._arr.len()
}

impl fmt::Display for Mx {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "AdjMx \n")?;
        for index in 0..self._arr.len() {
            write!(f, "[{}] ", index)?;
            for row_val in self._arr[index]._arr.iter() {
                write!(f, " {} ", row_val)?;
            }
            write!(f, "\n")?;
        }
        write!(f, "")
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn adj_matrix_create() {
        let mut adjmx = super::new(3);
        super::add_adj_item(&mut adjmx, 0, 1);
        super::add_adj_item(&mut adjmx, 0, 2);
        super::add_adj_item(&mut adjmx, 1, 0);
        super::add_adj_item(&mut adjmx, 2, 0);

        assert_eq!(adjmx._arr.len(), 3);
        assert_eq!(adjmx._arr[0]._arr.len(), 2);
        assert_eq!(adjmx._arr[1]._arr.len(), 1);
        assert_eq!(adjmx._arr[2]._arr.len(), 1);

    }

    #[test]
    fn adj_mx_degree_at_v() {
        let mut adjmx = super::new(5);
        super::add_adj_item(&mut adjmx, 0, 1);
        super::add_adj_item(&mut adjmx, 0, 2);
        super::add_adj_item(&mut adjmx, 0, 3);
        super::add_adj_item(&mut adjmx, 1, 0);
        super::add_adj_item(&mut adjmx, 2, 0);
        super::add_adj_item(&mut adjmx, 2, 3);
        super::add_adj_item(&mut adjmx, 3, 0);
        super::add_adj_item(&mut adjmx, 3, 2);

        assert_eq!(super::degree(&adjmx, 0), 3);
        assert_eq!(super::degree(&adjmx, 1), 1);
        assert_eq!(super::degree(&adjmx, 2), 2);
        assert_eq!(super::degree(&adjmx, 3), 2);
        assert_eq!(super::degree(&adjmx, 4), 0);

        assert_eq!(super::max_degree(&adjmx).unwrap(), 3);

        assert_eq!(super::min_degree(&adjmx).unwrap(), 0);

    }

    #[test]
    fn test() {
        let mut v = vec![2,3,5];
        v.iter().filter(|&i| i.eq(&22)).count();
    }

    #[test]
    fn test2() {
        let mut v = vec![false, false, true];
        println!("vec = {:?}", v[0]);
    }

    #[test]
    fn adj_mx_has_self_loop() {
        let mut adjmx = super::new(3);
        super::add_adj_item(&mut adjmx, 0, 1);
        super::add_adj_item(&mut adjmx, 0, 2);
        super::add_adj_item(&mut adjmx, 1, 0);
        super::add_adj_item(&mut adjmx, 2, 0);

        assert!(!super::has_self_loop(&adjmx));

        super::add_adj_item(&mut adjmx, 1, 1);
        assert!(super::has_self_loop(&adjmx));
    }

}