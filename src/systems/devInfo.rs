use macroquad::{color::BLACK, text::draw_text, time::get_fps};

use crate::Player;

pub fn dev_info_system(player: &Player) {
    draw_text("First Person Camera", 10.0, 20.0, 30.0, BLACK);
    draw_text(
        format!("FPS: {}", get_fps()).as_str(),
        10.0,
        44.0,
        30.0,
        BLACK,
    );
    draw_text(
        format!(
            "X: {:.2} Y: {:.2} Z: {:.2}",
            player.position.x, player.position.y, player.position.z
        )
        .as_str(),
        10.0,
        48.0 + 18.0,
        30.0,
        BLACK,
    );
    draw_text(
        format!("Press <TAB> to toggle mouse grab: {}", player.grabbed).as_str(),
        10.0,
        48.0 + 42.0,
        30.0,
        BLACK,
    );
    draw_text(
        format!(
            "Yaw: {:.2} Pitch: {:.2}",
            player.yaw.to_degrees(),
            player.pitch.to_degrees(),
        )
        .as_str(),
        10.0,
        48.0 + 58.0,
        30.0,
        BLACK,
    );
}
