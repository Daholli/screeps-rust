use log::info;
use screeps::{ErrorCode, HasPosition, ObjectId, Source};
use web_sys::console::info;

use crate::{
    movement::{MovementGoal, MovementProfile},
    task::TaskResult,
    worker::WorkerReference,
};

pub fn harvest_energy_forever(
    worker: &WorkerReference,
    target: &ObjectId<Source>,
    movement_profile: MovementProfile,
) -> TaskResult {
    match worker {
        WorkerReference::Creep(creep) => match target.resolve() {
            Some(source) => match creep.harvest(&source) {
                Ok(()) => TaskResult::StillWorking,
                Err(e) => match e {
                    ErrorCode::NotInRange => {
                        let move_goal = MovementGoal {
                            pos: source.pos(),
                            range: 1,
                            profile: movement_profile,
                            avoid_creeps: false,
                        };
                        TaskResult::MoveMeTo(move_goal)
                    }
                    ErrorCode::NoBodypart => {
                        creep
                            .suicide()
                            .expect("Failed to kill creep that was missing body parts");
                        TaskResult::Complete
                    }
                    e => {
                        info!("harvest failure: {:?}", e);
                        TaskResult::Complete
                    }
                },
            },
            None => TaskResult::Complete,
        },
        _ => panic!("unsupported worker type!"),
    }
}
