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

