mod goal;
mod path_state;

use crate::constants::{HIGH_CPU_THRESHOLD, LOW_BUCKET_THRESHOLD};
use crate::ShardState;
pub use goal::MovementGoal;
use log::warn;
pub use path_state::PathState;
use screeps::game;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum MovementProfile {
    // can move at full speed on swamp (either 5:1 move parts ratio, or
    // all parts are move/empty carry)
    SwampFiveToOne,
    // can move at full speed on plains (1:1 move ratio)
    PlainsOneToOne,
    // can only move once per tick on roads, weight them appropriately
    RoadsOneToTwo,
}

pub(crate) fn run_movement_and_remove_worker_refs(shard_state: &mut ShardState) {
    // creeps that are idle register themselves in this hashmap so that creeps
    // moving to their position can get them to swap positions as a simple
    // 'traffic management' mechanic (but pretty durable, absent pull() trains or immobile creeps)
    let mut idle_creeps = HashMap::new();

    // and creeps that are moving register where they're looking to move here
    // when they do, so that we can look for idle creeps at that location
    // to swap with
    let mut moving_creeps = HashMap::new();

    // check if CPU is high this tick or the bucket is low, we'll skip finding new paths if so
    let tick_cpu = game::cpu::get_used();
    let bucket_cpu = game::cpu::bucket();
    let cpu_critical = if tick_cpu > HIGH_CPU_THRESHOLD {
        warn!(
            "CPU usage high, will skip finding fresh paths: {}",
            tick_cpu
        );
        true
    } else if bucket_cpu < LOW_BUCKET_THRESHOLD {
        warn!(
            "CPU bucket low, will skip finding fresh paths: {}",
            bucket_cpu
        );
        true
    } else {
        false
    };

    // loop through all workers, removing their reference for use
    // during this movement step (or simply discarded in the case
    // of worker roles that can't move)
    for worker_state in shard_state.worker_state.values_mut() {
        // take the reference out of the worker
        if let Some(worker_reference) = worker_state.worker_reference.take() {
            // if the worker can't move, that's all we needed to do as end-of-tick cleanup
            if worker_state.role.can_move() && worker_reference.fatigue() == 0 {
                // it's a role that can move, let's consider it for movement
                let position = worker_reference.pos();
                // it can move - check if it has somewhere to be, and mark it as idle if not
                if let Some(movement_goal) = worker_state.movement_goal.take() {
                    // we have a goal; first check if it's met
                    if position.get_range_to(movement_goal.pos) <= movement_goal.range {
                        // goal is met! unset the path_state if there is one and idle
                        worker_state.path_state = None;
                        idle_creeps.insert(position, worker_reference);
                    } else {
                        // goal isn't met - let's see if there's a cached path that seems valid
                        let path_needed =
                            if let Some(mut path_state) = worker_state.path_state.take() {
                                // first call the function that updates the current position
                                // (or the stuck count if we didn't move)
                                path_state.check_if_moved_and_update_pos(position);

                                // check only for equality of the goal position as opposed to the whole goal
                                // so that changes in the avoid_creeps state don't invoke a repath
                                if path_state.goal.pos == movement_goal.pos
                                    && path_state.stuck_count <= STUCK_REPATH_THRESHOLD
                                {
                                    // still has the same goal as the cached path; we're ok
                                    // to simply move, retaining the path unless it's not returned
                                    worker_state.path_state = worker_reference.move_with_path(
                                        path_state,
                                        position,
                                        &mut moving_creeps,
                                    );
                                    false
                                } else {
                                    // the goal has changed or we're stuck - mark pathing as needed!
                                    true
                                }
                            } else {
                                // no cached path found, mark as needed
                                true
                            };

                        // if we need to path and we're in a CPU state to do it, do so
                        if path_needed && !cpu_critical {
                            let path_state = movement_goal.find_path_to(position);
                            worker_state.path_state = worker_reference.move_with_path(
                                path_state,
                                position,
                                &mut moving_creeps,
                            );
                        }

                        // put the goal back that we took, since the goal isn't yet met
                        worker_state.movement_goal = Some(movement_goal);
                    }
                } else {
                    // no goal, mark as idle!
                    idle_creeps.insert(position, worker_reference);
                }
            }
        } else {
            warn!("worker with no reference in move step!");
            continue;
        }
    }

    // look for idle creeps where we actively have creeps saying they intend to move
    for (dest_pos, moving_direction) in moving_creeps.iter() {
        if let Some(worker_reference) = idle_creeps.get(dest_pos) {
            worker_reference.swap_move(-*moving_direction)
        }
    }
}