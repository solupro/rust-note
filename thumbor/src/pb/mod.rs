use base64::{decode_config, encode_config, URL_SAFE_NO_PAD};
use photon_rs::transform::SamplingFilter;
use prost::Message;
use std::convert::TryFrom;

mod abi;
pub use abi::*;

impl ImageSpec {
  pub fn new(specs: Vec<Spec>) -> Self {
    Self { specs }
  }
}

// 生成字符串
impl From<&ImageSpec> for String {
  fn from(spec: &ImageSpec) -> Self {
    let data = spec.encode_to_vec();
    encode_config(data, URL_SAFE_NO_PAD)
  }
}

impl TryFrom<&str> for ImageSpec {
  type Error = anyhow::Error;

  fn try_from(value: &str) -> Result<Self, Self::Error> {
    let data = decode_config(value, URL_SAFE_NO_PAD)?;
    Ok(ImageSpec::decode(&data[..])?)
  }
}

// 辅助转换
impl filter::Filter {
  pub fn to_str(&self) -> Option<&'static str> {
    match self {
      filter::Filter::Unspecified => None,
      filter::Filter::Oceanic => Some("oceanic"),
      filter::Filter::Islands => Some("islands"),
      filter::Filter::Marine => Some("marine"),
    }
  }
}

impl From<resize::SimpleFilter> for SamplingFilter {
  fn from(v: resize::SimpleFilter) -> Self {
    match v {
      resize::SimpleFilter::Undefined => SamplingFilter::Nearest,
      resize::SimpleFilter::Nearest => SamplingFilter::Nearest,
      resize::SimpleFilter::Triangle => SamplingFilter::Triangle,
      resize::SimpleFilter::CatmullRom => SamplingFilter::CatmullRom,
      resize::SimpleFilter::Gaussian => SamplingFilter::Gaussian,
      resize::SimpleFilter::Lanczos3 => SamplingFilter::Lanczos3,
    }
  }
}

impl Spec {
  pub fn new_resize_seam_carve(width: u32, height: u32) -> Self {
    Self { 
      data: Some(spec::Data::Resize(Resize {
        width,
        height,
        rtype: resize::ResizeType::SeamCarve as i32,
        filter: resize::SimpleFilter::Undefined as i32,
      })),
    }
  }

  pub fn new_resize(width: u32, height: u32, filter: resize::SimpleFilter) -> Self {
    Self { 
      data: Some(spec::Data::Resize(Resize {
        width,
        height,
        rtype: resize::ResizeType::SeamCarve as i32,
        filter: filter as i32,
      })),
    }
  }

  pub fn new_filter(filter: filter::Filter) -> Self {
    Self {
      data: Some(spec::Data::Filter(Filter {
        filter: filter as i32,
      })),
    }
  }

  pub fn new_watermark(x: u32, y: u32) -> Self {
    Self { 
      data: Some(spec::Data::Watermark(
        Watermark{x, y}
      )),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::borrow::Borrow;
  use std::convert::TryInto;

  #[test]
  fn encode_spec_could_be_decode() {
    let spec1 = Spec::new_resize(600, 600, resize::SimpleFilter::CatmullRom);
    let spec2 = Spec::new_filter(filter::Filter::Marine);
    let image_spec = ImageSpec::new(vec![spec1, spec2]);
    let s: String = image_spec.borrow().into();
    assert_eq!(image_spec, s.as_str().try_into().unwrap());
  }
}