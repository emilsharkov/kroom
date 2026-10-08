use std::{any::TypeId, error::Error};

use daggy::NodeIndex;

use crate::{container::Container, dependency_graph::DependencyGraph, registration::Registration};

pub struct ContainerBuilder;

impl ContainerBuilder {
    pub fn build() -> Result<Container, Box<dyn Error>> {
        let mut container: Container = Container::new();

        // Eagerly initialize singletons via reverse topological sort of dependency graph DAG
        let dependency_graph: DependencyGraph = DependencyGraph::build()?;
        let topological_sort: Vec<NodeIndex> = dependency_graph.get_topological_sort()?;
        for node in topological_sort.iter().rev() {
            let (implementation_id, registration): (&TypeId, &Registration) = dependency_graph
                .get_registration_by_node(node)
                .expect("Node to have a registration");
            container.register_injectable(implementation_id, registration);
            println!("{:#?}",implementation_id);
            println!("{:#?}",registration);
        }

        Ok(container)
    }
}
