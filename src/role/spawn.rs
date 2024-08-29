use std::collections::HashSet;

use screeps::{RoomName, Store};

use crate::{role::WorkerRole, task::Task, worker::Worker};

pub struct Spawn {
    pub room_name: RoomName,
}

impl Worker for Spawn {
    fn find_task(&self, store: &Store, worker_roles: &HashSet<WorkerRole>) -> Task {
        todo!()
    }
}
