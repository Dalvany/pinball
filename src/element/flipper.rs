use bevy::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Component)]
pub(crate) enum Side {
    Left,
    Right,
}
