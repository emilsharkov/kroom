use std::{any::TypeId, collections::{HashMap, HashSet, VecDeque}, error::Error};

use daggy::{Dag, NodeIndex};

use crate::{injectable::Injectable, registration::{RegisteredType, Registration}};

pub struct ContainerValidator {
    dag: Dag<TypeId,()>,
    type_to_node_index: HashMap<TypeId,NodeIndex>,
    node_to_registration: HashMap<NodeIndex,Registration>
}

impl ContainerValidator {
    pub fn new() -> Self {
        ContainerValidator {
            dag: Dag::new(),
            type_to_node_index: HashMap::new(),
            node_to_registration: HashMap::new()
        }
    }

    pub fn validate(&mut self) -> Result<(),Box<dyn Error>> {
        self.initialize_nodes()?;
        self.initialize_edges()?;
        Ok(())
    }

    fn initialize_nodes(&mut self) -> Result<(),Box<dyn Error>> {
        for registration in inventory::iter::<Registration> {
            let interface_id: TypeId = registration.interface_id;
            let implementation_id: TypeId = registration.implementation_id;
            let interface_name: String = (registration.interface_name)();
            let implementation_name: String = (registration.implementation_name)();
            
            self
                .type_to_node_index
                .entry(interface_id)
                .or_insert_with(|| self.dag.add_node(interface_id));

            let interface_node_index: &NodeIndex = self.type_to_node_index
                .get(&interface_id)
                .ok_or_else(|| format!("Expected to find {} in DAG",interface_name))?;

            self.node_to_registration.insert(*interface_node_index,*registration);
            
            if interface_id != implementation_id {
                self
                    .type_to_node_index
                    .entry(implementation_id)
                    .or_insert_with(|| self.dag.add_node(implementation_id));   

                let implementation_node_index: &NodeIndex = self.type_to_node_index
                    .get(&implementation_id)
                    .ok_or_else(|| format!("Expected to find {} in DAG",implementation_name))?;

                self.node_to_registration.insert(*implementation_node_index,*registration);
            }
        }
        Ok(())
    }

    fn initialize_edges(&mut self) -> Result<(),Box<dyn Error>> {
        for registration in inventory::iter::<Registration> {
            let interface_id: TypeId = registration.interface_id;
            let implementation_id: TypeId = registration.implementation_id;
            let interface_name: String = (registration.interface_name)();
            let implementation_name: String = (registration.implementation_name)();
            let dependent_types: Vec<RegisteredType> = (registration.dependent_types)();

            let interface_node_index: &NodeIndex = self.type_to_node_index
                .get(&interface_id)
                .ok_or_else(|| format!("Expected to find {} in DAG",interface_name))?;

            let implementation_node_index: &NodeIndex = self.type_to_node_index
                .get(&implementation_id)
                .ok_or_else(|| format!("Expected to find {} in DAG",implementation_name))?;

            if implementation_node_index != interface_node_index {
                // Add edge between parent (interface_id) and child (implementation_id)
                self.dag
                    .add_edge(
                        *interface_node_index, 
                        *implementation_node_index, 
                        ()
                    )?;
            }

            // For each dependent type: add edge between parent (implementation_id) and (dependent_type)
            for dependent_type in dependent_types {
                let dependent_type_id: TypeId = dependent_type.id;
                let dependent_type_name: String = dependent_type.name;
                let dependent_type_node_index: &NodeIndex = self.type_to_node_index
                    .get(&dependent_type_id)
                    .ok_or_else(|| format!("Expected to find {} in DAG",dependent_type_name))?;
                self.dag
                    .add_edge(
                        *implementation_node_index, 
                        *dependent_type_node_index, 
                        ()
                    )?;
            } 
        }
        Ok(())
    }

    /* 
     If making an edge from older to newer causes a circular dependency, 
     then there exists an edge from newer to older.
     This function returns a string representation of the chain.
     Example: "older -> newer -> ... -> older "
    */
    fn get_circular_dependency_chain(&self, older: NodeIndex, newer: NodeIndex) -> String {
        let mut parent_to_child_nodes: HashMap<NodeIndex,Option<NodeIndex>> = HashMap::new();
        let mut node_queue: VecDeque<NodeIndex> = VecDeque::new();

        node_queue.in

        return "".to_string();
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