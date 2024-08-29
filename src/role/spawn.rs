use std::collections::HashSet;

use log::info;
use screeps::{find, game, HasPosition, Part, Room, RoomName, Source, Store, StructureSpawn};

use crate::{role::*, task::Task, worker::Worker};

#[derive(Eq, Hash, PartialEq)]
pub struct Spawn {
    pub room_name: RoomName,
}

impl Worker for Spawn {
    fn find_task(&self, store: &Store, worker_roles: &HashSet<WorkerRole>) -> Task {
        let room = game::rooms()
            .get(self.room_name)
            .expect("Did not find room for spawn.");

        let creep_to_spawn = Self::find_suitable_source(room, worker_roles);
        match creep_to_spawn {
            None => info!("Could not find source that is not harvested fully"),
            Some(creep) => {
                Task::SpawnCreep(creep);
            }
        }

        Task::IdleUntil(game::time() + 1000)
    }

    fn get_body_for_creep(&self, spawn: &StructureSpawn) -> Vec<Part> {
        panic!("Spawn has no body parts")
    }

    fn can_move(&self) -> bool {
        false
    }
}

impl Spawn {
    fn find_suitable_source(room: Room, worker_roles: &HashSet<WorkerRole>) -> Option<WorkerRole> {
        for source in room.find(find::SOURCES_ACTIVE, None) {
            let harvester_role = WorkerRole::Harvester(Harvester {
                source_position: source.pos(),
            });

            if !worker_roles.contains(&harvester_role) {
                return Some(harvester_role);
            }
        }
        None
    }
}
