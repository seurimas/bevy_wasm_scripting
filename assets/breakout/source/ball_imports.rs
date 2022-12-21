extern "C" {
    // XXX: I would really like to return a tuple or struct.
    // Need to figure out how that works properly with wasmer.
    pub fn get_velocity_x(me: i64) -> f32;
    pub fn get_velocity_y(me: i64) -> f32;
    pub fn set_velocity(me: i64, vx: f32, vy: f32);
}
