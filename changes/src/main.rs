use changes::*;

fn main() {
    let mut lights = ["living_room", "bedroom", "rest_room"].map(Light::new);

    println!("brightness = {}", lights[0].brightness);

    change_brightness(&mut lights, "living_room", 200);

    println!("new brightness = {}", lights[0].brightness);
}