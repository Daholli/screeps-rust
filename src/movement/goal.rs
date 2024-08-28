use crate::movement::MovementProfile;
use screeps::Position;

#[derive(Clone, Debug)]
pub struct MovementGoal {
    pub pos: Position,
    pub range: u32,
    pub profile: MovementProfile,
    pub avoid_creeps: bool,
}

impl MovementGoal {
    pub(crate) fn find_path_to(&self, p0: Position) -> _ {
        todo!()
    }
}