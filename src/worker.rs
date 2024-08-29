use std::collections::{HashMap, HashSet, VecDeque};

use enum_dispatch::enum_dispatch;
use log::warn;
use screeps::{
    find,
    game,
    Creep,
    HasId,
    HasPosition,
    MaybeHasId,
    ObjectId,
    Part,
    Position,
    SharedCreepProperties,
    Store,
    StructureObject,
    StructureSpawn,
    StructureTower,
};

use crate::{
    movement::{MovementGoal, MovementProfile, PathState},
    role::*,
    task::{Task, TaskResult},
    ShardState,
};

#[derive(Eq, Hash, PartialEq)]
pub enum WorkerId {
    Creep(ObjectId<Creep>),
    Spawn(ObjectId<StructureSpawn>),
    Tower(ObjectId<StructureTower>),
}

impl WorkerId {
    /// Resolve the WorkerId into a WorkerReference if it still exists
    pub fn resolve(&self) -> Option<WorkerReference> {
        match self {
            WorkerId::Creep(id) => id.resolve().map(WorkerReference::Creep),
            WorkerId::Spawn(id) => id.resolve().map(WorkerReference::Spawn),
            WorkerId::Tower(id) => id.resolve().map(WorkerReference::Tower),
        }
    }
}

pub enum WorkerReference {
    Creep(Creep),
    Spawn(StructureSpawn),
    Tower(StructureTower),
}

impl WorkerReference {
    pub(crate) fn move_with_path(
        &self,
        path_state: PathState,
        position: Position,
        position_hashmap: &mut HashMap<Position, u32>,
    ) -> Option<PathState> {
        todo!()
    }
}

impl WorkerReference {
    pub fn pos(&self) -> Position {
        match self {
            WorkerReference::Creep(o) => o.pos(),
            WorkerReference::Spawn(o) => o.pos(),
            WorkerReference::Tower(o) => o.pos(),
        }
    }

    pub fn fatigue(&self) -> u32 {
        match self {
            WorkerReference::Creep(o) => o.fatigue(),
            _ => 0,
        }
    }

    pub fn store(&self) -> Store {
        match self {
            WorkerReference::Creep(o) => o.store(),
            WorkerReference::Spawn(o) => o.store(),
            WorkerReference::Tower(o) => o.store(),
        }
    }
}

#[enum_dispatch]
pub trait Worker {
    fn find_task(&self, store: &Store, worker_role: &HashSet<WorkerRole>) -> Task;
    fn get_body_for_creep(&self, spawn: &StructureSpawn) -> Vec<Part>;
    fn get_movement_profile(&self) -> MovementProfile {
        MovementProfile::RoadsOneToTwo
    }
    fn can_move(&self) -> bool {
        true
    }
}

pub struct WorkerState {
    pub role: WorkerRole,
    pub task_queue: VecDeque<Task>,
    pub worker_reference: Option<WorkerReference>,
    pub movement_goal: Option<MovementGoal>,
    pub path_state: Option<PathState>,
}

impl WorkerState {
    pub fn new_with_role_and_reference(
        role: WorkerRole,
        worker_reference: WorkerReference,
        task_queue: VecDeque<Task>,
    ) -> WorkerState {
        WorkerState {
            role,
            task_queue,
            worker_reference: Some(worker_reference),
            movement_goal: None,
            path_state: None,
        }
    }
}

pub fn scan_and_register_creeps(shard_state: &mut ShardState) {
    for creep in game::creeps().values() {
        // this function is called at the start of tick before any tasks, so we can simply assume
        // every creep has an id; if spawning had run then id-free creeps would be a possibility.
        let id = WorkerId::Creep(creep.try_id().expect("expected creep to have id!"));

        // update the reference if there's already a worker for this creep id,
        // or parse the name and add it if it's not there
        shard_state
            .worker_state
            .entry(id)
            .and_modify(|worker_state| {
                // worker exists!
                // cloning the creep here so it's not moved and unavailable to the or_insert_with
                // branch below
                worker_state.worker_reference = Some(WorkerReference::Creep(creep.clone()))
            })
            .or_insert_with(|| {
                let creep_name: String = creep.name().into();
                match serde_json::from_str(&creep_name) {
                    Ok(role) => {
                        let task_queue = if creep.spawning() {
                            let mut queue = VecDeque::new();
                            queue.push_front(Task::WaitToSpawn);
                            queue
                        } else {
                            VecDeque::new()
                        };
                        // add to hashset where we track which roles are filled by active workers
                        shard_state.worker_roles.insert(role);
                        // then create the state struct
                        WorkerState::new_with_role_and_reference(
                            role,
                            WorkerReference::Creep(creep),
                            task_queue,
                        )
                    }
                    Err(e) => {
                        warn!("couldn't parse creep name {}: {:?}", creep_name, e);
                        // special case, don't insert to the hashset where we track roles, since
                        // this isn't a valid role
                        let role = WorkerRole::Invalid(Invalid {});
                        WorkerState {
                            role,
                            task_queue: VecDeque::new(),
                            worker_reference: Some(WorkerReference::Creep(creep)),
                            movement_goal: None,
                            path_state: None,
                        }
                    }
                }
            });
    }
}

pub fn scan_and_register_structures(shard_state: &mut ShardState) {
    for room in game::rooms().values() {
        // narrowing the scan down to just rooms that are owned currently,
        // as all structure types that are 'workers' in this bot can only
        // function in owned rooms
        let owned = room
            .controller()
            .map_or(false, |controller| controller.my());

        if owned {
            let room_name = room.name();

            for structure in room.find(find::MY_STRUCTURES, None) {
                match structure {
                    StructureObject::StructureSpawn(spawn) => {
                        let id = WorkerId::Spawn(spawn.id());
                        let role = WorkerRole::from(Spawn { room: room_name });
                        let worker_state = WorkerState::new_with_role_and_reference(
                            role,
                            WorkerReference::Spawn(spawn),
                            VecDeque::new(),
                        );
                        shard_state.worker_state.insert(id, worker_state);
                    }
                    StructureObject::StructureTower(tower) => {
                        let id = WorkerId::Tower(tower.id());
                        let role = WorkerRole::Tower(Tower { room: room_name });
                        let worker_state = WorkerState::new_with_role_and_reference(
                            role,
                            WorkerReference::Tower(tower),
                            VecDeque::new(),
                        );
                        shard_state.worker_state.insert(id, worker_state);
                    }
                    // we don't make workers for any other structure types!
                    _ => {}
                }
            }
        }
    }
}

pub fn run_workers(shard_state: &mut ShardState) {
    // track which worker ids can't resolve and should be removed from the hashmap after iteration
    let mut remove_worker_ids = vec![];
    let mut remove_worker_roles = vec![];

    for (worker_id, worker_state) in shard_state.worker_state.iter_mut() {
        if worker_state.worker_reference.is_none() {
            // hasn't resolved yet this tick; try to resolve and if we still can't,
            // mark the worker for removal and skip it
            match worker_id.resolve() {
                Some(resolved_worker) => {
                    worker_state.worker_reference = Some(resolved_worker);
                }
                None => {
                    // couldn't resolve the worker, mark it for removal
                    remove_worker_ids.push(*worker_id);
                    remove_worker_roles.push(worker_state.role);
                    continue;
                }
            }
        }

        // we've either resolved the worker or continue has jumped out of the loop, unwrap
        let worker_ref = worker_state.worker_reference.as_ref().unwrap();
        let movement_profile = worker_state.role.get_movement_profile();

        match worker_state.task_queue.pop_front() {
            Some(task) => {
                // we've got a task, run it!
                match task.run_task(worker_ref, movement_profile) {
                    // nothing to do if complete, already popped
                    TaskResult::Complete => {}
                    TaskResult::StillWorking => {
                        worker_state.task_queue.push_front(task);
                    }
                    TaskResult::MoveMeTo(move_goal) => {
                        worker_state.movement_goal = Some(move_goal);
                        worker_state.task_queue.push_front(task)
                    }
                    TaskResult::AddTaskToFront(result_task) => {
                        // add the result task in front after re-adding the existing task
                        worker_state.task_queue.push_front(task);
                        worker_state.task_queue.push_front(result_task);
                    }
                    TaskResult::CompleteAddTaskToFront(result_task) => {
                        worker_state.task_queue.push_front(result_task);
                    }
                    TaskResult::CompleteAddTaskToBack(result_task) => {
                        worker_state.task_queue.push_back(result_task);
                    }
                    TaskResult::DestroyWorker => {
                        remove_worker_ids.push(*worker_id);
                        remove_worker_roles.push(worker_state.role);
                    }
                }
            }
            None => {
                // no task in queue, let's find one (even if it's just to go idle)
                // include the worker's store and the worker role hashset
                let new_task = worker_state
                    .role
                    .find_task(&worker_ref.store(), &shard_state.worker_roles);
                match new_task.run_task(worker_ref, movement_profile) {
                    TaskResult::Complete => {
                        warn!("instantly completed new task, unexpected: {:?}", new_task)
                    }
                    TaskResult::StillWorking => {
                        worker_state.task_queue.push_front(new_task);
                    }
                    TaskResult::MoveMeTo(move_goal) => {
                        worker_state.movement_goal = Some(move_goal);
                        worker_state.task_queue.push_front(new_task)
                    }
                    TaskResult::AddTaskToFront(result_task) => {
                        // add the result task in front after re-adding the existing task
                        worker_state.task_queue.push_front(new_task);
                        worker_state.task_queue.push_front(result_task);
                    }
                    TaskResult::CompleteAddTaskToFront(result_task) => {
                        worker_state.task_queue.push_front(result_task);
                    }
                    TaskResult::CompleteAddTaskToBack(result_task) => {
                        worker_state.task_queue.push_back(result_task);
                    }
                    TaskResult::DestroyWorker => {
                        remove_worker_ids.push(*worker_id);
                        remove_worker_roles.push(worker_state.role);
                    }
                }
            }
        }
    }

    for id in remove_worker_ids {
        shard_state.worker_state.remove(&id);
    }

    for role in remove_worker_roles {
        shard_state.worker_roles.remove(&role);
    }
}
