mod harvest;
mod r#move;
mod spawn;

use screeps::{ObjectId, Position, Source};

use crate::role::WorkerRole;

pub enum TaskResult {
    Complete,
    StillWorking,
}
pub enum Task {
    SpawnCreep(WorkerRole),
    HarvestEnergyForever(ObjectId<Source>),
    IdleUntil(u32),
    MoveToPosition(Position, i32),
}
