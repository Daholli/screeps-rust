use std::collections::HashMap;

use log::warn;
use screeps::{game, Position};

use crate::{
    constants::{HIGH_CPU_THRESHOLD, LOW_BUCKET_THRESHOLD},
    role::*,
    worker::{Worker, WorkerReference},
    ShardState,
};

pub fn run_movement_and_remove_worker_refs(shard_state: &mut ShardState) {
    // creeps that are idle register themselves in this hashmap so that creeps
    // moving to their position can get them to swap positions as a simple
    // 'traffic management' mechanic (but pretty durable, absent pull() trains or immobile creeps)
    let mut idle_creeps: HashMap<Position, WorkerReference> = HashMap::new();

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

    for worker_state in shard_state.worker_state.values_mut() {
        if let Some(worker_reference) = worker_state.worker_reference.take() {
            if worker_state.role.can_move() && worker_reference.fatigue() == 0 {}
        }
    }
}
