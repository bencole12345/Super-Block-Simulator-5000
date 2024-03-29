use std::cmp::min;

use nalgebra::Vector3;

use sbs5k_core::chunk;
use sbs5k_engine::{lighting, FogParameters};

use crate::Args;

/// Set up the initial lighting parameters for the scene
pub(crate) fn make_scene_lighting() -> lighting::SceneLighting {
    let point_lights = vec![];
    let global_light = lighting::GlobalLight {
        direction: Vector3::new(1.0, 1.2, 1.5).normalize(),
        colour: Vector3::new(1.0, 1.0, 1.0),
        intensity: 0.5,
    };

    lighting::SceneLighting {
        point_lights,
        global_light,
    }
}

/// Set up the fog parameters
pub(crate) fn make_fog_parameters(config: &Args) -> FogParameters {
    let near_chunks = config.render_distance - 1;
    let far_chunks = config.render_distance;
    let chunk_size = min(chunk::CHUNK_WIDTH, chunk::CHUNK_DEPTH) as u32;

    let near_distance = (near_chunks * chunk_size) as f32;
    let far_distance = (far_chunks * chunk_size) as f32;

    FogParameters {
        start_threshold: near_distance,
        end_threshold: far_distance,
    }
}
