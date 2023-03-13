#[derive(Hash, Copy, Clone, Debug, PartialEq, Eq)]
pub enum AppState {
    Paused,
    AwaitInput,
    PlayerTurn,
    MonsterTurn,
}
