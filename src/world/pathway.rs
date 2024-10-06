use bevy::prelude::*;

use crate::map::Segment;

use super::{structure::Pathway, Rotation};


// Should we separate pathways from buildings? as pathways shouln't have an `Area` component, but a `Segment` component.
// #[derive(Bundle)]
// pub struct PathwayBundle {
//     pathway: Pathway,
//     rotation: Rotation,
//     segment: Segment
// }