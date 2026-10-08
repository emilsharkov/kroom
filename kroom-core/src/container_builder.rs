use std::error::Error;

use daggy::NodeIndex;

use crate::{
    container::Container, dependency_graph::DependencyGraph, registration::Registration,
    scope::Scope,
};

pub struct ContainerBuilder;

impl ContainerBuilder {
    pub fn build() -> Result<Container, Box<dyn Error>> {
        let mut container: Container = Container::new();

        // Eagerly initialize singletons via reverse topological sort of dependency graph DAG
        let dependency_graph: DependencyGraph = DependencyGraph::build()?;
        let topological_sort: Vec<NodeIndex> = dependency_graph.get_topological_sort()?;
        for node in topological_sort.iter().rev() {
            let registration: &Registration = dependency_graph
                .get_registration(node)
                .expect("Node to have a registration");
            let scope: Scope = (registration.scope)();
            if scope == Scope::Singleton {
                container.register_singleton(registration);
            }
        }

        Ok(container)
    }
}
