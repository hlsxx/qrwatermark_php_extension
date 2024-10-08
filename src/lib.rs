use ext_php_rs::prelude::*;
use qrwatermark::configs::image_config::ImageConfigBuilder;
use qrwatermark::configs::logo_config::LogoConfigBuilder;
use qrwatermark::traits::builder::Builder;
use qrwatermark::QrWatermark;

#[php_function]
fn qrwatermark_generate(
  qr_code_text: &str,
  logo_path: &str,
  result_image_path: &str,
  color: Vec<u8>
) -> bool {
  let image_config = ImageConfigBuilder::new()
    .color(color)
    .build();

  let logo_config =  LogoConfigBuilder::new()
    .width(70)
    .height(70)
    .build();

  let mut qrw = QrWatermark::new(qr_code_text)
    .logo(logo_path)
    .image_config(image_config)
    .logo_config(logo_config);

  match qrw.save_as_png(result_image_path) {
    Ok(_) => true,
    Err(_) => false
  }
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
  module
}
