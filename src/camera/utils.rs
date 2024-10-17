use std::f64::consts::PI;

// 3D座標を表す構造体
struct Point3D {
    x: f64,
    y: f64,
    z: f64,
}

// 2D座標を表す構造体
struct Point2D {
    x: f64,
    y: f64,
}

// 透視投影を行う関数
fn perspective_projection(point: &Point3D, focal_length: f64) -> Point2D {
    // 透視投影の数式
    let projected_x = (focal_length * point.x) / point.z;
    let projected_y = (focal_length * point.y) / point.z;

    // 2D座標として返す
    Point2D {
        x: projected_x,
        y: projected_y,
    }
}

// ガウスのレンズの公式を計算する関数
fn gaussian_lens_formula(object_distance: f64, image_distance: f64) -> f64 {
    // 物体距離と像距離が有効な場合、焦点距離を計算
    let focal_length = 1.0 / ((1.0 / object_distance) + (1.0 / image_distance));
    focal_length
}

// 撮影画角を計算する関数
fn calculate_aov(sensor_size: f64, focal_length: f64) -> f64 {
    // 画角をラジアン(弧度法)で計算
    let theta = 2.0 * (sensor_size / (2.0 * focal_length)).atan();
    
    // ラジアンを度に変換
    theta * 180.0 / PI
}

// 35mm換算焦点距離を計算する関数
fn calculate_35mm_equivalent(focal_length: f64, sensor_diagonal: f64) -> f64 {
    // 35mmフルサイズセンサーの対角線長は43.3mm
    let full_frame_diagonal = 43.3;

    // 35mm換算焦点距離の計算
    focal_length * (full_frame_diagonal / sensor_diagonal)
}

// シャッタースピードを計算
fn calculate_shutter_speed(distance_to_subject: f64, subject_speed: f64, 
    focal_length: f64, sensor_size: f64, pixel_count: u32) -> f64 {

    let t_s = (sensor_size * distance_to_subject) / (subject_speed * focal_length * pixel_count as f64);
    t_s
}
