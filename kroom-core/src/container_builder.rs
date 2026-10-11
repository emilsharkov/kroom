use std::{any::TypeId, error::Error};

use daggy::NodeIndex;

use crate::{container::Container, container_validator::ContainerValidator, dependency_graph::DependencyGraph, registration::Registration, scope::Scope};

pub struct ContainerBuilder;

impl ContainerBuilder {
    pub fn build() -> Result<Container, Box<dyn Error>> {
        let mut container: Container = Container::new();
        for registration in inventory::iter::<Registration> {
            let interface_id: TypeId = registration.interface_id;
            container.register_injectable(&interface_id, registration);
        }
        
        let dependency_graph: DependencyGraph = DependencyGraph::build()?;
        let container_validator: ContainerValidator = ContainerValidator::new();
        container_validator.validate_dependency_graph(&dependency_graph)?;

        // Eagerly initializesingletons via reverse topological sort of dependency graph DAG
        let topological_sort: Vec<NodeIndex> = dependency_graph.get_topological_sort()?;
        for node in topological_sort.iter().rev() {
            let registrations: &Vec<Registration> = dependency_graph
                .get_registrations_by_node(node)
                .expect("Node to have a registration");
            for registration in registrations {
                let scope: Scope = (registration.scope)();
                if scope == Scope::Singleton {
                    container.register_singleton(registration);
                }
            }
        }

        Ok(container)
    }
}
