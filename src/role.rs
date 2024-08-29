use std::collections::HashSet;

use enum_dispatch::enum_dispatch;
use screeps::{Part, Store, StructureSpawn};
use serde::{Deserialize, Serialize};

use crate::{task::Task, worker::Worker};

mod builder;
mod harvester_source;
mod hauler;
mod upgrader;

pub use self::{
    harvester_source::HarvesterSource,
    // hauler::Hauler,
    // upgrader::Upgrader
};

#[derive(Eq, Hash, PartialEq)]
pub struct Invalid {}

impl Worker for Invalid {
    fn find_task(&self, store: &Store, worker_role: &HashSet<WorkerRole>) -> Task {
        Task::IdleUntil(u32::MAX)
    }

    fn get_body_for_creep(&self, spawn: &StructureSpawn) -> Vec<Part> {
        panic!("Cannot spawn invalid workers!")
    }
}
#[enum_dispatch(Worker)]
#[derive(Eq, Hash, PartialEq, Serialize, Deserialize)]
pub enum WorkerRole {
    HarvesterSource(HarvesterSource),
    Invalid(Invalid),
}
