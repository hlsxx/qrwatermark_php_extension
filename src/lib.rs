use ext_php_rs::prelude::*;
use qrwatermark::QrWatermark;

#[php_function]
fn qrwatermark_generate(
  qr_code_text: &str,
  logo_path: &str,
  result_image_path: &str,
) -> bool {
  let mut qrw = QrWatermark::new(qr_code_text)
  .logo(logo_path);

  match qrw.save_as_png(result_image_path) {
    Ok(_) => true,
    Err(_) => false
  }
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
  module
}
