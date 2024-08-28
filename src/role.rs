use std::thread::Builder;

mod harvester_source;
mod builder;
mod upgrader;
mod hauler;

pub enum WorkerRole {
    Builder(Builder),
    Upgrader(),
}

impl WorkerRole {
    pub(crate) fn get_movement_profile(&self) -> _ {
        todo!()
    }
}

impl WorkerRole {
    pub(crate) fn can_move(&self) -> bool {
        todo!()
    }
}