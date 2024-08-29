use screeps::{game, ObjectId, Position, Source, StructureController};

use crate::{
    movement::{MovementGoal, MovementProfile},
    worker::{Worker, WorkerReference},
};

mod harvest;
mod spawn;

pub enum TaskResult {
    Complete,
    StillWorking,
    MoveMeTo(MovementGoal),
    AddTaskToFront(Task),
    CompleteAddTaskToFront(Task),
    CompleteAddTaskToBack(Task),
    DestroyWorker,
}

#[derive(Debug)]
pub enum Task {
    IdleUntil(u32),
    MoveToPosition(Position, u32),
    HarvestEnergyUntilFull(ObjectId<Source>),
    HarvestEnergyForever(ObjectId<Source>),
    Upgrade(ObjectId<StructureController>),
    WaitToSpawn,
}

impl Task {
    pub fn run_task(
        &self,
        worker: &WorkerReference,
        movement_profile: MovementProfile,
    ) -> TaskResult {
        match self {
            Task::IdleUntil(tick) => {
                if game::time() >= *tick {
                    TaskResult::Complete
                } else {
                    TaskResult::StillWorking
                }
            }
            Task::MoveToPosition(position, range) => {
                if worker.pos().get_range_to(*position) <= *range {
                    TaskResult::Complete
                } else {
                    TaskResult::MoveMeTo(MovementGoal {
                        pos: *position,
                        range: *range,
                        profile: movement_profile,
                        avoid_creeps: false,
                    })
                }
            }
            Task::HarvestEnergyUntilFull(id) => {
                todo!("not implemented")
            }
            Task::HarvestEnergyForever(id) => {
                harvest::harvest_energy_forever(worker, id, movement_profile)
            }
            Task::Upgrade(id) => {
                todo!("not implemented")
            }
            Task::WaitToSpawn => spawn::wait_to_spawn(worker),
        }
    }
}
