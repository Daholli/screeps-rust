use std::collections::HashSet;

use screeps::{
    look,
    HasId,
    Part,
    Part::{Move, Work},
    Position,
    Store,
    StructureSpawn,
};

use crate::{role::WorkerRole, task::Task, worker::Worker};

#[derive(Eq, Hash, PartialEq)]
pub struct Harvester {
    pub source_position: Position,
}

impl Worker for Harvester {
    fn find_task(&self, store: &Store, worker_roles: &HashSet<WorkerRole>) -> Task {
        match self.source_position.look_for(look::SOURCES) {
            Ok(sources) => match sources.first() {
                Some(source) => Task::HarvestEnergyForever(source.id()),
                None => Task::MoveToPosition(self.source_position, 1),
            },
            Err(_) => Task::MoveToPosition(self.source_position, 1),
        }
    }

    fn get_body_for_creep(&self, spawn: &StructureSpawn) -> Vec<Part> {
        vec![Work, Work, Move]
    }
}
