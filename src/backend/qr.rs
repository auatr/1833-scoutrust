use qrcode::QrCode;
use image::{ImageBuffer, Luma};
use crate::config::data::{Match};

pub fn gen_match_iamge(use_match: Match) -> ImageBuffer<Luma<u8>, Vec<u8>> {
    let match_string = ron::to_string(&use_match).expect("Failed to encode match to string");

    let code = QrCode::new(match_string).expect("Failed to create QR code");
    
    code.render::<Luma<u8>>().build()
}

pub fn gen_rest_image() {} // Make for the future: generate iamge with Firebase REST api link