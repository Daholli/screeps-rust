mod harvester;
mod spawn;

use std::collections::HashSet;

use enum_dispatch::enum_dispatch;
use screeps::{Part, Store, StructureSpawn};

pub use self::{harvester::Harvester, spawn::Spawn};
use crate::{task::Task, worker::Worker};

#[enum_dispatch(Worker)]
#[derive(Eq, Hash, PartialEq)]
pub enum WorkerRole {
    Spawn(Spawn),
    Harvester(Harvester),

    Invalid(Invalid),
}

#[derive(Eq, Hash, PartialEq)]
pub struct Invalid {}

impl Worker for Invalid {
    fn find_task(&self, store: &Store, worker_roles: &HashSet<WorkerRole>) -> Task {
        Task::IdleUntil(u32::MAX)
    }

    fn get_body_for_creep(&self, spawn: &StructureSpawn) -> Vec<Part> {
        panic!("Cannot spawn invalid creep")
    }
}
