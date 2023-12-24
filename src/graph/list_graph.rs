use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt::Debug,
    hash::Hash,
};

pub struct GraphAdjList<T: Debug + Eq + Hash + Clone> {
    pub adj_list: HashMap<T, Vec<T>>,
}

impl<T: Debug + Eq + Hash + Clone> GraphAdjList<T> {
    pub fn new(edges: Vec<[T; 2]>) -> Self {
        let mut graph = Self {
            adj_list: HashMap::new(),
        };

        for [v1, v2] in edges {
            graph.add_vertex(v1.clone());
            graph.add_vertex(v2.clone());
            graph.add_edge(v1, v2);
        }
        graph
    }

    pub fn size(&self) -> usize {
        self.adj_list.len()
    }

    pub fn add_edge(&mut self, vet1: T, vet2: T) {
        if !self.adj_list.contains_key(&vet1) || !self.adj_list.contains_key(&vet2) || vet1 == vet2
        {
            panic!("value error");
        }

        let list1 = self.adj_list.get_mut(&vet1).unwrap();
        if !list1.contains(&vet2) {
            list1.push(vet2.clone());
        }

        let list2 = self.adj_list.get_mut(&vet2).unwrap();
        if !list2.contains(&vet1) {
            list2.push(vet1);
        }
    }

    pub fn remove_edge(&mut self, vet1: T, vet2: T) {
        if !self.adj_list.contains_key(&vet1) || !self.adj_list.contains_key(&vet2) || vet1 == vet2
        {
            panic!("value error");
        }

        let list1 = self.adj_list.get_mut(&vet1).unwrap();
        list1.retain(|item| item != &vet2);

        let list2 = self.adj_list.get_mut(&vet2).unwrap();
        list2.retain(|item| item != &vet1);
    }

    pub fn add_vertex(&mut self, vet: T) {
        if self.adj_list.contains_key(&vet) {
            return;
        }

        self.adj_list.insert(vet, vec![]);
    }

    pub fn remove_vertex(&mut self, vet: T) {
        if !self.adj_list.contains_key(&vet) {
            panic!("value error");
        }

        self.adj_list.remove(&vet);

        for list in self.adj_list.values_mut() {
            list.retain(|v| v != &vet);
        }
    }

    pub fn print(&self) {
        println!("邻接表 =");
        for (vertex, list) in &self.adj_list {
            println!("{:?}: {:?},", vertex, list);
        }
    }

    pub fn bfs(&self) -> Vec<T> {
        let mut result = vec![];
        if self.size() == 0 {
            return result;
        }

        let first = self.adj_list.keys().nth(0).unwrap();
        let mut que = VecDeque::new();
        que.push_back(first);

        let mut visited = HashSet::new();
        visited.insert(first);

        while !que.is_empty() {
            let vertex = que.pop_front().unwrap();
            result.push(vertex.clone());

            for adj in self.adj_list.get(vertex).unwrap() {
                if visited.contains(adj) {
                    continue;
                }

                que.push_back(adj);
                visited.insert(adj);
            }
        }

        result
    }

    pub fn dfs(&self) -> Vec<T> {
        let mut res = vec![];
        if let Some(first) = self.adj_list.keys().nth(0) {
            let mut visited = HashSet::new();
            self._dfs(&mut res, &mut visited, first);
        }
        res
    }

    fn _dfs<'a>(&'a self, res: &mut Vec<T>, visited: &mut HashSet<&'a T>, start_vertex: &'a T) {
        res.push(start_vertex.clone());
        visited.insert(start_vertex);

        if let Some(adj_list) = self.adj_list.get(&start_vertex) {
            for adj in adj_list {
                if visited.contains(adj) {
                    continue;
                }

                self._dfs(res, visited, adj);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init() {
        let mut graph = GraphAdjList::new(vec![[1, 3], [1, 5], [3, 2], [2, 5], [5, 4], [2, 4]]);
        graph.print();

        graph.add_vertex(6);
        graph.print();

        graph.add_edge(4, 6);
        graph.print();

        println!("bfs: {:?}", graph.bfs());
        println!("dfs: {:?}", graph.dfs());
    }
}
