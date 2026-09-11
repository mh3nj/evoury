use std::collections::{HashMap, HashSet, VecDeque};
use uuid::Uuid;

pub struct DependencyGraph {
    edges: HashMap<Uuid, Vec<Uuid>>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self {
            edges: HashMap::new(),
        }
    }

    pub fn add_dependency(&mut self, asset_id: Uuid, depends_on_id: Uuid) {
        self.edges
            .entry(asset_id)
            .or_default()
            .push(depends_on_id);
    }

    pub fn remove_dependency(&mut self, asset_id: &Uuid, depends_on_id: &Uuid) {
        if let Some(deps) = self.edges.get_mut(asset_id) {
            deps.retain(|d| d != depends_on_id);
            if deps.is_empty() {
                self.edges.remove(asset_id);
            }
        }
    }

    pub fn dependencies_of(&self, asset_id: &Uuid) -> Vec<Uuid> {
        self.edges
            .get(asset_id)
            .cloned()
            .unwrap_or_default()
    }

    pub fn dependents_of(&self, asset_id: &Uuid) -> Vec<Uuid> {
        self.edges
            .iter()
            .filter(|(_, deps)| deps.contains(asset_id))
            .map(|(id, _)| *id)
            .collect()
    }

    pub fn transitive_dependencies(&self, asset_id: &Uuid) -> Vec<Uuid> {
        let mut visited = HashSet::new();
        let mut result = Vec::new();
        let mut stack = vec![*asset_id];
        while let Some(current) = stack.pop() {
            if let Some(deps) = self.edges.get(&current) {
                for dep in deps {
                    if visited.insert(*dep) {
                        result.push(*dep);
                        stack.push(*dep);
                    }
                }
            }
        }
        result
    }

    pub fn transitive_dependents(&self, asset_id: &Uuid) -> Vec<Uuid> {
        let mut visited = HashSet::new();
        let mut result = Vec::new();
        let mut queue = VecDeque::new();
        queue.push_back(*asset_id);
        while let Some(current) = queue.pop_front() {
            let dependents: Vec<Uuid> = self
                .edges
                .iter()
                .filter(|(_, deps)| deps.contains(&current))
                .map(|(id, _)| *id)
                .collect();
            for dep in dependents {
                if visited.insert(dep) {
                    result.push(dep);
                    queue.push_back(dep);
                }
            }
        }
        result
    }

    pub fn has_cycle(&self) -> bool {
        let mut visited = HashSet::new();
        let mut rec_stack = HashSet::new();
        let nodes: Vec<Uuid> = self.edges.keys().cloned().collect();
        for node in &nodes {
            if self.dfs_cycle(*node, &mut visited, &mut rec_stack) {
                return true;
            }
        }
        false
    }

    fn dfs_cycle(
        &self,
        node: Uuid,
        visited: &mut HashSet<Uuid>,
        rec_stack: &mut HashSet<Uuid>,
    ) -> bool {
        if rec_stack.contains(&node) {
            return true;
        }
        if visited.contains(&node) {
            return false;
        }
        visited.insert(node);
        rec_stack.insert(node);
        if let Some(deps) = self.edges.get(&node) {
            for dep in deps {
                if self.dfs_cycle(*dep, visited, rec_stack) {
                    return true;
                }
            }
        }
        rec_stack.remove(&node);
        false
    }

    pub fn topological_sort(&self) -> Vec<Uuid> {
        if self.has_cycle() {
            return Vec::new();
        }
        let mut visited = HashSet::new();
        let mut result = Vec::new();
        let nodes: Vec<Uuid> = self.edges.keys().cloned().collect();
        for node in &nodes {
            self.dfs_topological(*node, &mut visited, &mut result);
        }
        result.reverse();
        result
    }

    fn dfs_topological(&self, node: Uuid, visited: &mut HashSet<Uuid>, result: &mut Vec<Uuid>) {
        if visited.contains(&node) {
            return;
        }
        visited.insert(node);
        if let Some(deps) = self.edges.get(&node) {
            for dep in deps {
                self.dfs_topological(*dep, visited, result);
            }
        }
        result.push(node);
    }
}
