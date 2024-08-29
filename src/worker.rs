use std::collections::HashSet;

use enum_dispatch::enum_dispatch;
use screeps::Store;

use crate::{role::*, task::Task, ShardState};

pub enum WorkerId {}

pub enum WorkerReference {}

#[enum_dispatch]
pub trait Worker {
    fn find_task(&self, store: &Store, worker_roles: &HashSet<WorkerRole>) -> Task;
}

pub struct WorkerState {}

pub(crate) fn scan_and_register_creeps(shard_state: &mut ShardState) {
    todo!()
}

pub(crate) fn scan_and_register_structures(shard_state: &mut ShardState) {
    todo!()
}

pub(crate) fn run_workers(shard_state: &mut ShardState) {
    todo!()
}
