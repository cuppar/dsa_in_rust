use std::fmt::Debug;

pub struct GraphAdjMatrix<T: Debug> {
    pub vertices: Vec<T>,
    pub adj_mat: Vec<Vec<i32>>,
}

impl<T: Debug> GraphAdjMatrix<T> {
    pub fn new(vertices: Vec<T>, edges: Vec<[usize; 2]>) -> Self {
        let mut graph = Self {
            vertices: vec![],
            adj_mat: vec![],
        };

        for v in vertices {
            graph.add_vertex(v);
        }

        for [i, j] in edges {
            graph.add_edge(i, j);
        }
        graph
    }

    pub fn size(&self) -> usize {
        self.vertices.len()
    }

    pub fn add_vertex(&mut self, val: T) {
        let n = self.size();
        self.vertices.push(val);
        self.adj_mat.push(vec![0; n]);
        for row in &mut self.adj_mat {
            row.push(0);
        }
    }

    pub fn remove_vertex(&mut self, index: usize) {
        if index >= self.size() {
            panic!("out of index");
        }
        self.vertices.remove(index);

        self.adj_mat.remove(index);

        for row in &mut self.adj_mat {
            row.remove(index);
        }
    }

    pub fn add_edge(&mut self, i: usize, j: usize) {
        if i >= self.size() || j >= self.size() || i == j {
            panic!("index error")
        }

        self.adj_mat[i][j] = 1;
        self.adj_mat[j][i] = 1;
    }

    pub fn remove_edge(&mut self, i: usize, j: usize) {
        if i >= self.size() || j >= self.size() || i == j {
            panic!("index error")
        }

        self.adj_mat[i][j] = 0;
        self.adj_mat[j][i] = 0;
    }

    pub fn print(&self) {
        println!("顶点列表 = {:?}", self.vertices);
        println!("邻接矩阵 =");
        println!("[");
        for row in &self.adj_mat {
            println!("  {:?},", row);
        }
        println!("]")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let mut graph = GraphAdjMatrix::new(
            vec![1, 3, 2, 5, 4],
            vec![[0, 1], [0, 3], [1, 2], [2, 3], [4, 3], [2, 4]],
        );
        graph.print();

        graph.add_vertex(6);
        graph.print();

        graph.add_edge(4, 5);
        graph.print();
    }
}
