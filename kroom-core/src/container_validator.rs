use std::error::Error;

use daggy::NodeIndex;

use crate::dependency_graph::DependencyGraph;

pub struct ContainerValidator {}

impl ContainerValidator {
    pub fn new() -> Self {
        ContainerValidator {}
    }

    pub fn validate(&mut self, dependency_graph: &DependencyGraph) -> Result<(), Box<dyn Error>> {
        let topological_sort: Vec<NodeIndex> = dependency_graph.get_topological_sort()?;
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
