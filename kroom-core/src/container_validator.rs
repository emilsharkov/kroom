use std::{any::TypeId, collections::{HashMap, HashSet, VecDeque}, error::Error};

use daggy::{Dag, NodeIndex, Walker};

use crate::{dependency_graph::DependencyGraph, injectable::Injectable, registration::{RegisteredType, Registration}};

pub struct ContainerValidator { }

impl ContainerValidator {
    pub fn new() -> Self {
        ContainerValidator { }
    }

    pub fn validate(&mut self, dependency_graph: &DependencyGraph) -> Result<(),Box<dyn Error>> {
        // dependency_graph.
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