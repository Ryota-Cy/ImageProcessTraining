use std::path::Path;
use image_process_training::filters::grayscale::to_grayscale;
use image::ImageReader;
use image::RgbImage;
use show_image::{create_window, event, ImageView, ImageInfo};

#[show_image::main]
fn main() {
    // JPEG画像の読み込み
    let img_path = Path::new("./img/Lenna.jpg");
    let img = ImageReader::open(&img_path).unwrap().decode().unwrap();

    // グレースケールに変換
    let gray_img = to_grayscale(&img);

    // ウィンドウを作成して画像を表示
    show_image_window(&gray_img);
}


// 画像をウィンドウに表示する関数
fn show_image_window(img: &RgbImage) {
    // ウィンドウを作成
    let window = create_window("Grayscale Image", Default::default())
        .expect("Failed to create window");

    // ImageView を作成
    let view = ImageView::new(ImageInfo::rgb8(img.width(), img.height()), img.as_raw());

    // ウィンドウに画像を表示
    let _ = window.set_image("image-001", view);

    // イベントループでEnterキーを待機してウィンドウを閉じる
    for event in window.event_channel().unwrap() {
        if let event::WindowEvent::KeyboardInput(event) = event {
            if let Some(key_code) = event.input.key_code {
                if key_code == event::VirtualKeyCode::Return && event.input.state.is_pressed() {
                    break;
                }
            }
        }
    }
}