#[derive(Hash, Copy, Clone, Debug, PartialEq, Eq)]
pub enum TurnState {
    Paused,
    AwaitInput,
    PlayerTurn,
    MonsterTurn,
}
