pub mod honda;
pub mod toyota;

use revview_core::vehicle::{GENERIC, VehicleProfile};

/// Order matters only for the dropdown. Saved settings use `id`, not the index.
pub const ALL: &[&VehicleProfile] = &[&GENERIC, &toyota::PRIUS_GEN3, &honda::CIVIC_GEN9];

pub fn index_of(id: &str) -> usize {
    ALL.iter().position(|p| p.id == id).unwrap_or(0)
}
