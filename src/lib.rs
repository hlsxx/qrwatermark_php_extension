use ext_php_rs::prelude::*;
use std::convert::TryInto;
use qrwatermark::configs::image_config::ImageConfigBuilder;
use qrwatermark::configs::logo_config::LogoConfigBuilder;
use qrwatermark::traits::builder::Builder;
use qrwatermark::QrWatermark;

fn vec_to_array(vec_data: Vec<u8>) -> [u8; 3] {
  return vec_data.try_into().unwrap_or([0, 0, 0]);
}

#[php_function]
fn qrwatermark_generate(
  qr_code_text: &str,
  logo_path: &str,
  result_image_path: &str,
  color: Option<Vec<u8>>,
  background_color: Option<Vec<u8>>,
  is_gradient_enabled: Option<bool>,
  logo_width: Option<u32>,
  logo_height: Option<u32>
) -> bool {
  let mut image_config = ImageConfigBuilder::new();

  // Custom color
  if let Some(color_data) = color {
    image_config = image_config.color(vec_to_array(color_data));
  }

  // Custom background color
  if let Some(background_color_data) = background_color {
    image_config = image_config.background_color(vec_to_array(background_color_data));
  }

  // Is gradient enabled
  if let Some(is_gradient_enabled_data) = is_gradient_enabled {
    if is_gradient_enabled_data {
      image_config = image_config.is_gradient_enabled();
    }
  }

  let mut logo_config =  LogoConfigBuilder::new();

  // Custom logo width
  if let Some(logo_width_data) = logo_width {
    logo_config = logo_config.width(logo_width_data);
  }

  // Custom logo height
  if let Some(logo_height_data) = logo_height {
    logo_config = logo_config.height(logo_height_data);
  }

  let mut qrw = QrWatermark::new(qr_code_text)
    .logo(logo_path)
    .image_config(image_config.build())
    .logo_config(logo_config.build());

  match qrw.save_as_png(result_image_path) {
    Ok(_) => true,
    Err(_) => false
  }
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
  module
}
