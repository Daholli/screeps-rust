use std::collections::HashSet;

use enum_dispatch::enum_dispatch;
use screeps::{
    Creep,
    HasPosition,
    ObjectId,
    Part,
    Position,
    Store,
    StructureSpawn,
    StructureTower,
};

use crate::{role::*, task::Task, ShardState};

pub enum WorkerId {
    Creep(ObjectId<Creep>),
    Spawn(ObjectId<StructureSpawn>),
    Tower(ObjectId<StructureTower>),
}

impl WorkerId {
    pub fn resolve(&self) -> Option<WorkerReference> {
        match self {
            WorkerId::Creep(id) => id.resolve().map(WorkerReference::Creep),
            WorkerId::Spawn(id) => id.resolve().map(WorkerReference::Spawn),
            WorkerId::Tower(id) => id.resolve().map(WorkerReference::Tower),
        }
    }
}
pub enum WorkerReference {
    Creep(Creep),
    Spawn(StructureSpawn),
    Tower(StructureTower),
}
impl WorkerReference {
    /// Get the worker's current position
    pub fn pos(&self) -> Position {
        match self {
            WorkerReference::Creep(o) => o.pos(),
            WorkerReference::Spawn(o) => o.pos(),
            WorkerReference::Tower(o) => o.pos(),
        }
    }

    /// Get the worker's fatigue (for the movement library)
    pub fn fatigue(&self) -> u32 {
        match self {
            WorkerReference::Creep(o) => o.fatigue(),
            _ => 0,
        }
    }

    /// Get the worker's store (for task finding)
    pub fn store(&self) -> Store {
        match self {
            WorkerReference::Creep(o) => o.store(),
            WorkerReference::Spawn(o) => o.store(),
            WorkerReference::Tower(o) => o.store(),
        }
    }
}

#[enum_dispatch]
pub trait Worker {
    fn find_task(&self, store: &Store, worker_roles: &HashSet<WorkerRole>) -> Task;
    fn get_body_for_creep(&self, spawn: &StructureSpawn) -> Vec<Part>;
    fn can_move(&self) -> bool {
        true
    }
}

pub struct WorkerState {
    pub worker_reference: Option<WorkerReference>,
    pub role: WorkerRole,
}

pub fn scan_and_register_creeps(shard_state: &mut ShardState) {
    todo!()
}

pub fn scan_and_register_structures(shard_state: &mut ShardState) {
    todo!()
}

pub fn run_workers(shard_state: &mut ShardState) {
    todo!()
}
