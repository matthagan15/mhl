use std::collections::HashSet;



use crate::structs::*;
use crate::traits::*;

/// Applies an input graph to an input vector a specified amount of times.
pub fn walk<N: HgNode>(
    start: SparseVector<N>,
    walk_operator: &SparseGraph<N>,
    num_steps: usize,
) -> SparseVector<N> {
    let mut ret = start;
    for _ in 0..num_steps {
        ret = walk_operator.map_vec(ret);
    }
    ret
}

/// first pass at basic BFS, probably something done incorrectly
pub fn bfs_base<B: HgBasis>(graph: &GeneroGraph<B>, start: &B, steps: usize) -> Vec<HgPath<B>> {
    // TODO: change this to a dequeue.
    let mut visited = HashSet::new();
    let start_path = HgPath::new(start.clone());
    let mut frontier = vec![start_path];
    let mut completed = Vec::new();
    while frontier.len() > 0 {
        let cur_path = frontier.pop().expect("loop should not execute if empty.");
        visited.insert(cur_path.last_basis());
        let new_paths = cur_path.extend(graph);
        for path in new_paths.into_iter() {
            if path.len() < steps && visited.contains(&path.last_basis()) == false {
                frontier.insert(0, path);
            } else if path.len() == steps {
                completed.push(path);
            }
        }
    }
    completed
}

// Thoughts on making an enum of walkers?
// Thoughts on making iterators of walkers?
/// First pass at basic DFS, probably something done incorrectly.
pub fn dfs_base<B: HgBasis>(graph: &GeneroGraph<B>, start: &B, steps: usize) -> Vec<HgPath<B>> {
    let mut visited = HashSet::new();
    let start_path = HgPath::new(start.clone());
    let mut frontier = vec![start_path];
    let mut completed = Vec::new();
    while frontier.len() > 0 {
        let cur_path = frontier.pop().expect("loop should not execute if empty.");
        visited.insert(cur_path.last_basis());
        let new_paths = cur_path.extend(graph);
        for path in new_paths.into_iter() {
            if path.len() < steps && visited.contains(&path.last_basis()) == false {
                frontier.push(path);
            } else if path.len() == steps {
                completed.push(path);
            }
        }
    }
    completed
}

/// Constructs a random walk graph out of a specified input graph.
pub fn compute_probabilistic_walk_graph<N: HgNode>(_graph: &SparseGraph<N>) -> SparseGraph<N> {
    SparseGraph::<N>::new()
}

/// Where cut computations would go
pub fn compute_cut<N: HgNode>(selected_nodes: HashSet<N>, graph: &SparseGraph<N>) {
    let mut pot_edges = HashSet::new();
    for node in selected_nodes.iter() {
        let new_edges = graph.get_outbound_edges(node);
        for e in new_edges {
            pot_edges.insert(e);
        }
    }
}

mod test {
    

    

    #[test]
    fn test_bfs() {
        let mut hg = HGraph::new();
        let mut nodes = hg.create_nodes(10);
        nodes.sort();
        let start = &nodes[0..2];
        let b1 = [nodes[0], nodes[1], nodes[2]];
        let b2 = [nodes[0], nodes[1], nodes[3]];
        let b3 = [nodes[3]];
        let b4 = [nodes[4]];
        let b5 = [nodes[5]];
        hg.create_edge(start, &b1, 1., crate::EdgeDirection::Directed);
        hg.create_edge(start, &b2, 1., crate::EdgeDirection::Directed);
        hg.create_edge(&b1, &b3, 1., crate::EdgeDirection::Directed);
        hg.create_edge(&b1, &b4, 1., crate::EdgeDirection::Directed);
        hg.create_edge(&b2, &b3, 1., crate::EdgeDirection::Directed);
        hg.create_edge(&b2, &b5, 1., crate::EdgeDirection::Directed);
        println!("nodes: {:#?}", nodes);
        println!("graph: {:#?}", hg.graph);
        println!(
            "bfs: {:#?}",
            bfs_base(
                &hg.graph,
                &SparseBasis::from(start.iter().cloned().collect()),
                2
            )
        );
    }
}
