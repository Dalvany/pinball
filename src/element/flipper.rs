use bevy::prelude::*;

use crate::PAD_DEAD_ZONE;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Side {
    Left,
    Right,
}

/// Component to keep track of the angle
#[derive(Copy, Clone, Debug, Component)]
pub(crate) struct AngleRange {
    side: Side,
    current_angle: f32,
    range: f32,
}

impl AngleRange {
    pub(crate) fn new(side: Side, min_angle: f32, max_angle: f32) -> Self {
        Self {
            side,
            current_angle: min_angle,
            range: max_angle - min_angle,
        }
    }

    pub(crate) fn side(&self) -> Side {
        self.side
    }

    pub(crate) fn rotate(&mut self, force: f32) -> f32 {
        let force = if force < PAD_DEAD_ZONE {
            0.
        } else {
            force - PAD_DEAD_ZONE
        };

        if force == 0. && self.current_angle > 0. {
            let old = self.current_angle;
            self.current_angle = 0.;
            -old
        } else if force > 0. && self.current_angle < self.range {
            self.current_angle = self.range;
            self.range
        } else {
            0.
        }
    }
}
