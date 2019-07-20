/// Game data like options, etc...
/// What goes here: updated rarely, mostly safe to read.

#[derive(Default)]
pub struct GameData {
    pub current_turn: i32,
    pub state_change_request: Option<StateChangeRequest>,
}

#[allow(dead_code)]
pub enum StateChangeRequest {
    ResetMenu,
    QuitGame,
}
