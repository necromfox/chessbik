use bevy::prelude::*;

use crate::{
    commons::{AvailableMovesStorage, SelectedPieceReference},
    AppLabels,
};

mod indication;
mod indicator;

pub use indicator::AvailableMovesIndicator;

pub struct AvailableMovesIndicationPlugin;

impl Plugin for AvailableMovesIndicationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedPieceReference>();
        app.init_resource::<AvailableMovesIndicator>();
        app.init_resource::<AvailableMovesStorage>();
        app.add_system(
            indication::system
                .label(AppLabels::Indication)
                .after(AppLabels::Selection),
        );
    }
}
