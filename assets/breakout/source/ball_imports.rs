// This is a transparent struct around 64, for use with entity ids.
// f64 is the best type to cooperate with wasm-bindgen for web builds.
// See: https://github.com/rustwasm/wasm-bindgen/issues/35
#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct EntityId(f64);

extern "C" {
    // Ideally, this would just be one function call, but...
    // BUG (wasmer): https://github.com/wasmerio/wasmer/issues/3447
    pub fn get_velocity_x(me: EntityId) -> f32;
    pub fn get_velocity_y(me: EntityId) -> f32;
    pub fn set_velocity(me: EntityId, vx: f32, vy: f32);
}
