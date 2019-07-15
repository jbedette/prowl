/// Game data like options, etc...
/// What goes here: updated rarely, mostly safe to read.

#[derive(Default)]
pub struct GameData {
    pub current_turn: i32,
    pub switch_state: Option<StateChangeRequest>,
}

pub enum StateChangeRequest {
    ResetMenu,
    QuitGame,
}
