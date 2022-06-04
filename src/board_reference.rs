use chessbik_board::PiecePosition;

chessbik_commons::derive_wrapper!(
    #[derive(bevy::prelude::Component, Clone, Copy)]
    pub struct BoardReference(PiecePosition);
);

impl Default for BoardReference {
    fn default() -> Self {
        Self(PiecePosition(0usize))
    }
}