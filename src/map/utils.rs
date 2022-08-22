use crate::map::*;

pub fn wrap_borders_system(
    windows: ResMut<Windows>,
    mut query: Query<&mut Transform, With<Creature>>,
) {
    if let Some(window) = windows.get_primary() {
        let width = window.width();
        let height = window.height();

        for mut transform in query.iter_mut() {
            if transform.translation.x >= width / 2.0 {
                transform.translation.x = -width / 2.0 + 1.0;
            } else if transform.translation.x <= -width / 2.0 {
                transform.translation.x = width / 2.0 - 1.0;
            }
            if transform.translation.y >= height / 2.0 {
                transform.translation.y = -height / 2.0 + 1.0;
            } else if transform.translation.y <= -height / 2.0 {
                transform.translation.y = height / 2.0 - 1.0;
            }
        }
    }
}
