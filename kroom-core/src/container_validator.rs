use std::{any::TypeId, error::Error};

use daggy::{Dag, NodeIndex, Walker};

use crate::{dependency_graph::DependencyGraph, registration::Registration, scope::Scope};

pub struct ContainerValidator {}

impl ContainerValidator {
    pub fn new() -> Self {
        ContainerValidator {}
    }

    pub fn validate_dependency_graph(&self, dependency_graph: &DependencyGraph) -> Result<(), Box<dyn Error>> {
        let dag: &Dag<TypeId,()> = dependency_graph.get_dag();
        let topological_sort: Vec<NodeIndex> = dependency_graph.get_topological_sort()?;

        for parent_node in topological_sort {
            let parent_registration: &Registration = dependency_graph
                .get_registrations_by_node(&parent_node)
                .ok_or("Expected each node to have a registration")?
                .first()
                .ok_or("Expected to find at least one registration per node")?;
            let parent_scope: Scope = (parent_registration.scope)();
            let parent_interface_name: String = (parent_registration.interface_name)();
            
            for (_, child_node) in dag.children(parent_node).iter(dag) {
                let child_registration: &Registration = dependency_graph
                    .get_registrations_by_node(&child_node)
                    .ok_or("Expected each node to have a registration")?
                    .first()
                    .ok_or("Expected to find at least one registration per node")?;
                let child_scope: Scope = (child_registration.scope)();
                let child_interface_name: String = (child_registration.interface_name)();
                match (parent_scope, child_scope) {
                    (Scope::Singleton, Scope::Transient) => {
                        return Err(format!(
                            "Singleton '{}' cannot depend on transient '{}'",
                            parent_interface_name, child_interface_name
                        ).into());
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }
}

// Car has multi injected wheels and a steering wheel
// Car
//     -> Wheel -> BrownWheel
//     -> Wheel -> BlackWheel
//     -> Wheel -> GreyWheel
//     -> Wheel -> WhiteWheel
//     -> Steering -> SquareSteering
//
//
// singleton -> singleton
// transitive -> singleton
// transitive -> transitive
//
//
// Validation Requirements
// * each dependency is present in dag
// * no cycles in dependencies
// * verify scope rules are followed
// * verify that single injected dependencies don't have multiple implementations
