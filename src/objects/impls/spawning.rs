use crate::{Creep, ReturnCode};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// Details of the creep being spawned currently that can be addressed by the [`StructureSpawn.spawning`] property.
    #[wasm_bindgen]
    pub type Spawning;

    /// The creep being spawned.
    #[wasm_bindgen(method, getter)]
    pub fn creep(this: &Spawning) -> Creep;

    /// Time needed in total to complete the spawning.
    #[wasm_bindgen(method, getter = needTime)]
    pub fn need_time(this: &Spawning) -> u32;

    /// Remaining time to complete the spawning.
    #[wasm_bindgen(method, getter = remainingTime)]
    pub fn remaining_time(this: &Spawning) -> u32;

    /// Cancel spawning immediately. Energy spent on spawning is not returned.
    #[wasm_bindgen(method)]
    pub fn cancel(this: &Spawning) -> ReturnCode;
}
