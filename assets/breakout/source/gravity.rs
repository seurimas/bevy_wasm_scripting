mod ball_imports;
use ball_imports::*;

const G: f32 = 100.0;

#[no_mangle]
pub unsafe extern "C" fn on_update(me: EntityId, delta_seconds: f32) {
    let vx = get_velocity_x(me);
    let vy = get_velocity_y(me);
    let vy = vy - delta_seconds * G;
    set_velocity(me, vx, vy);
}
