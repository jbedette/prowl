use specs::{Component, VecStorage};
use specs_derive::Component;

#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
pub struct AI {
    pub goal: Option<Goal>,
}

#[derive(Debug)]
pub enum Goal {
    MoveRandom,
}

impl AI {
    pub fn with_goal(goal: Goal) -> Self {
        let goal = Some(goal);
        Self { goal }
    }
}
