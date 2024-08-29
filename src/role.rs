mod spawn;

use enum_dispatch::enum_dispatch;

pub use self::spawn::Spawn;
#[enum_dispatch(Worker)]
pub enum WorkerRole {
    Spawn(Spawn),
}
