#![feature(extract_if)]

use std::collections::{HashMap, HashSet};

use slab_tree::{NodeId, Tree, TreeBuilder};

advent_of_code::solution!(6);

#[derive(PartialEq, Debug)]
struct OrbitTree<'a> {
    inner: Tree<&'a str>,
    node_ids: HashMap<&'a str, NodeId>,
    root_id: NodeId,
}

impl<'a> OrbitTree<'a> {
    fn from_str(s: &'a str) -> Self {
        let mut inner = TreeBuilder::new()
            .with_root("COM")
            .with_capacity(s.lines().count())
            .build();
        let root_id = inner.root_id().unwrap();
        let mut node_ids = HashMap::new();
        node_ids.insert("COM", root_id);
        let mut items = s
            .lines()
            .map(|l| l.split_once(')').unwrap())
            .collect::<Vec<_>>();
        let mut just_inserted = vec![(root_id, "COM")];
        while !items.is_empty() {
            let mut newly_inserted = Vec::new();
            for (parent_id, parent_name) in just_inserted {
                let mut parent_node = inner.get_mut(parent_id).unwrap();
                for (_, child) in items.extract_if(|(p, _)| *p == parent_name) {
                    let new_child_id = parent_node.append(child).node_id();
                    node_ids.insert(child, new_child_id);
                    newly_inserted.push((new_child_id, child));
                }
            }
            just_inserted = newly_inserted;
        }
        Self {
            inner,
            node_ids,
            root_id,
        }
    }

    fn num_orbits(&self) -> usize {
        self.inner
            .root()
            .unwrap()
            .traverse_level_order()
            .map(|n| n.ancestors().count())
            .sum()
    }

    fn num_transfers(&self, start: &str, end: &str) -> usize {
        let mut start = self.inner.get(self.node_ids[start]).unwrap();
        let mut end = self.inner.get(self.node_ids[end]).unwrap();
        let depths = self.dfs();
        let mut i = 0;
        while depths[&start.node_id()] != depths[&end.node_id()] {
            if depths[&start.node_id()] > depths[&end.node_id()] {
                let parent = start.parent().unwrap().node_id();
                start = self.inner.get(parent).unwrap();
            } else {
                let parent = end.parent().unwrap().node_id();
                end = self.inner.get(parent).unwrap();
            }
            i += 1;
        }
        while start.node_id() != end.node_id() {
            let start_parent = start.parent().unwrap().node_id();
            let end_parent = end.parent().unwrap().node_id();
            start = self.inner.get(start_parent).unwrap();
            end = self.inner.get(end_parent).unwrap();
            i += 2;
        }
        i - 2
    }

    fn dfs(&self) -> HashMap<NodeId, usize> {
        let mut out = HashMap::with_capacity(self.node_ids.len());
        let root_id = self.root_id;
        out.insert(root_id, 0);
        let mut visited = HashSet::with_capacity(self.node_ids.len());
        self.dfs_inner(root_id, &mut out, &mut visited);
        out
    }

    fn dfs_inner(
        &self,
        v: NodeId,
        depth: &mut HashMap<NodeId, usize>,
        visited: &mut HashSet<NodeId>,
    ) {
        visited.insert(v);
        let cur_depth = depth[&v];
        for node in self.inner.get(v).unwrap().children().map(|n| n.node_id()) {
            if visited.contains(&node) {
                continue;
            }
            depth.insert(node, cur_depth + 1);
            self.dfs_inner(node, depth, visited);
        }
    }
}

pub fn part_one(input: &str) -> Option<usize> {
    let tree = OrbitTree::from_str(input);
    Some(tree.num_orbits())
}

pub fn part_two(input: &str) -> Option<usize> {
    let tree = OrbitTree::from_str(input);
    Some(tree.num_transfers("YOU", "SAN"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(54));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4));
    }
}
