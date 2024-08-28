use crate::{
    role::WorkerRole,
    task::Task,
    worker::Worker,
};
use log::info;
use screeps::{look, ErrorCode, HasId, Part, Part::{Move, Work}, Position, ResourceType::Energy, Source, Store, StructureSpawn};
use std::collections::HashSet;

pub struct HarvesterSource {
    pub source_position: Position,
}

impl Worker for HarvesterSource {
    fn find_task(&self, store: &Store, worker_role: &HashSet<WorkerRole>) -> Task {
        match self.source_position.look_for(look::SOURCES) {
            Ok(sources) => {
                match sources.first() {
                    Some(source) => Task::HarvestEnergyForever(source.id()),
                    None => Task::MoveToPosition(self.source_position, 1)
                }
            }
            Err(_) => Task::MoveToPosition(self.source_position, 1)
        }
    }

    fn get_body_for_creep(&self, spawn: &StructureSpawn) -> Vec<Part> {
        let energy = spawn.store().get_used_capacity(Some(Energy));
        info!("spawn has {energy} energy");
        vec![Work, Work, Move]
    }
}