#![cfg_attr(coverage_nightly, feature(coverage_attribute))]

mod wasm;

use bevy::{asset::AssetMetaCheck, prelude::*};

#[cfg_attr(coverage_nightly, coverage(off))]
pub fn run() {
    App::new()
        .insert_resource(AssetMetaCheck::Never)
        .add_plugins(wasm::get_plugins())
        .run();
}
