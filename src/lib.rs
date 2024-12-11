use ext_php_rs::prelude::*;
use std::convert::TryInto;
use std::error;
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
  result_image_path: &str,
  logo_path: Option<String>,
  color: Option<Vec<u8>>,
  color_gradient: Option<Vec<Vec<u8>>>,
  background_color: Option<Vec<u8>>,
  is_gradient_enabled: Option<bool>,
  logo_width: Option<u32>,
  logo_height: Option<u32>,
  pixel_size: Option<u32>
) -> Result<bool, PhpException> {
  let mut image_config = ImageConfigBuilder::new()
    .pixel_size(pixel_size.unwrap_or(10));

  // Custom color
  if let Some(color_data) = color {
    image_config = image_config.color(vec_to_array(color_data));
  }

  // Custom gradient color
  if let Some(color_gradient_data) = color_gradient {
    if color_gradient_data.len() == 2 {
      let rgbs_tuple = (vec_to_array(color_gradient_data[0].clone()), vec_to_array(color_gradient_data[1].clone()));
      image_config = image_config.color_gradient(rgbs_tuple);
    }
  }

  // Custom background color
  if let Some(background_color_data) = background_color {
    image_config = image_config.background_color(vec_to_array(background_color_data));
  }

  // Is gradient enabled
  if let Some(is_gradient_enabled_data) = is_gradient_enabled {
    if is_gradient_enabled_data {
      image_config = image_config.is_auto_gradient_enabled();
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
    .image_config(image_config.build())
    .logo_config(logo_config.build());

  if let Some(logo_path_data) = &logo_path {
    qrw = qrw.logo(logo_path_data);
  }

  if let Err(err) = qrw.save_as_image(result_image_path) {
    return Err(PhpException::default(err.to_string()));
  }

  Ok(true)
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
  module
}
