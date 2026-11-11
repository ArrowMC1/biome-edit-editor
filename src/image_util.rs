use crate::image::Image;


// Function Filled In With An LLM 
// TODO: Double Check

pub fn create_shaded_relief_map(
    result_img: &mut Image,
    biome_map: &Vec<u32>,
    height_map: &Vec<u8>,
    img_length: i32,
    img_width: i32
) {
    // ==============================
    // CONFIGURATION
    // ==============================

    // Light direction
    let light_x: f32 = -0.5;
    let light_y: f32 = -0.5;
    let light_z: f32 = 1.0;

    // Terrain exaggeration
    let height_strength: f32 = 15.0;

    // Lighting
    let ambient_strength: f32 = 0.35;
    let directional_strength: f32 = 0.85;

    // Overall brightness
    let brightness_multiplier: f32 = 1.0;

    // Lighting contrast
    let contrast: f32 = 1.0;

    // ==============================
    // INITIALIZATION
    // ==============================

    let result_vec = &mut result_img.pixels;

    let width = img_width as usize;
    let height = img_length as usize;

    result_vec.resize(biome_map.len(), 0);

    let light_length = (
        light_x * light_x +
        light_y * light_y +
        light_z * light_z
    ).sqrt();

    let lx = light_x / light_length;
    let ly = light_y / light_length;
    let lz = light_z / light_length;

    for y in 0..height {
        for x in 0..width {
            let i = y * width + x;

            let get_height = |x: usize, y: usize| -> f32 {
                height_map[y * width + x] as f32 / 255.0
            };

            // Clamp edges
            let left   = x.saturating_sub(1);
            let right  = (x + 1).min(width - 1);
            let top    = y.saturating_sub(1);
            let bottom = (y + 1).min(height - 1);

            // Sobel gradient
            let tl = get_height(left, top);
            let tc = get_height(x, top);
            let tr = get_height(right, top);

            let ml = get_height(left, y);
            let mr = get_height(right, y);

            let bl = get_height(left, bottom);
            let bc = get_height(x, bottom);
            let br = get_height(right, bottom);

            let dx = (tr + 2.0 * mr + br) - (tl + 2.0 * ml + bl);
            let dy = (bl + 2.0 * bc + br) - (tl + 2.0 * tc + tr);

            // Surface normal
            let nx = -dx * height_strength;
            let ny = -dy * height_strength;
            let nz = 1.0;

            let normal_length = (
                nx * nx +
                ny * ny +
                nz * nz
            ).sqrt();

            let nx = nx / normal_length;
            let ny = ny / normal_length;
            let nz = nz / normal_length;

            // Directional lighting
            let diffuse = (nx * lx + ny * ly + nz * lz).max(0.0);

            // Combine ambient and directional lighting
            let mut brightness =
                ambient_strength +
                diffuse * directional_strength;

            // Adjust contrast around neutral brightness (1.0)
            brightness = 1.0 + (brightness - 1.0) * contrast;

            // Overall brightness
            brightness *= brightness_multiplier;

            let color = biome_map[i];

            let r = ((color >> 24) & 0xFF) as f32;
            let g = ((color >> 16) & 0xFF) as f32;
            let b = ((color >> 8) & 0xFF) as f32;
            let a = color & 0xFF;

            let r = (r * brightness).clamp(0.0, 255.0) as u32;
            let g = (g * brightness).clamp(0.0, 255.0) as u32;
            let b = (b * brightness).clamp(0.0, 255.0) as u32;

            result_vec[i] = (r << 24) | (g << 16) | (b << 8) | a;
        }
    }
}


pub fn change_noise_in_circle(
    target_map: &mut Vec<u16>,
    center_x: usize,
    center_z: usize,
    radius: f32,
    strength: f32,
    falloff: f32,
) {
    let width = crate::IMG_SIZE as usize;
    let height = crate::IMG_SIZE as usize;
    if width == 0 || height == 0 || radius <= 0.0 {
        return;
    }
    let min_x = (center_x as f32 - radius).max(0.0) as usize;
    let max_x = ((center_x as f32 + radius) as usize).min(width - 1);
    let min_z = (center_z as f32 - radius).max(0.0) as usize;
    let max_z = ((center_z as f32 + radius) as usize).min(height - 1);
    let radius_squared = radius * radius;
    for z in min_z..=max_z {
        for x in min_x..=max_x {
            // Bounds checking
            let index = z * width + x;
            if index >= crate::IMG_SIZE as usize * crate::IMG_SIZE as usize {
                continue;
            }
            let dx = x as f32 - center_x as f32;
            let dz = z as f32 - center_z as f32;
            let distance_squared = dx * dx + dz * dz;
            if distance_squared > radius_squared {
                continue;
            }
            let normalized_distance = distance_squared.sqrt() / radius;
            let effect = (1.0 - normalized_distance).powf(falloff);
            let change = (strength
                * effect
                * u16::MAX as f32) as u16;
            target_map[index] = target_map[index].saturating_add(change);
        }
    }
}


#[inline]
pub fn float_to_u16(mut val: f32) -> u16 {
    val = val.clamp(-2.0, 2.0);
    val += 2.0;
    val *= 65535.0 / 4.0;

    val.clamp(0.0, 65535.0) as u16
}

#[inline]
pub fn u16_to_float(val: u16) -> f32 {
    (val as f32 * 4.0 / 65535.0) - 2.0
}