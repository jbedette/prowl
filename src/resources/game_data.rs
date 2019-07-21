/// Game data like options, etc...
/// What goes here: updated rarely, mostly safe to read.

#[derive(Default)]
pub struct GameData {
    pub current_turn: i32,
    pub state_change_request: Option<StateChangeRequest>,
}

/// Used to tell the game to 'change state', by running another turn or pausing
/// or quitting.
#[allow(dead_code)]
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum StateChangeRequest {
    NextTurn,
    ResetMenu,
    QuitGame,
    WaitForUI,
}
