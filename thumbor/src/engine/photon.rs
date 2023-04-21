use supper::{Engine, SpecTransform};
use crate::pb::*;
use anyhow::{Result, Ok};
use bytes::Bytes;
use image::{
    DynamicImage,
    ImageBuffer,
    ImageOutputFormat, buffer
};
use photon_rs::{
    effects,
    filters,
    multiple,
    native::open_image_from_bytes,
    transform,
    PhotonImage,
};
use std::convert::TryFrom;

lazy_static! {
    // 预先把水印文件加载为静态变量
    static ref WATERMARK: PhotonImage = {
        // 这里你需要把我 github 项目下的对应图片拷贝到你的根目录
        // 在编译的时候 include_bytes! 宏会直接把文件读入编译后的二进制
        let data = include_bytes!("../../rust-logo.png");
        let watermark = open_image_from_bytes(data).unwrap();
        transform::resize(&watermark, 64, 64, transform::SamplingFilter::Nearest)
    };
}

// 我们目前支持 Photon engine
pub struct Photon(PhotonImage);

// 从 Bytes 转换成 Photon 结构
impl TryFrom<Bytes> for Photon {
    type Error = anyhow::Error;

    fn try_from(data: Bytes) -> Result<Self, Self::Error> {
        Ok(Self(open_image_from_bytes(&data)?))
    }
}

impl Engine for Photon {
    fn apply(&mut self, specs: &[Spec]) {
        for spec in specs.iter() {
            match spec.data {
                Some(spec::Data::Crop(ref v)) => self.transform(v),
                Some(spec::Data::Contrast(ref v)) => self.transform(v),
                Some(spec::Data::Filter(ref v)) => self.transform(v),
                Some(spec::Data::Fliph(ref v)) => self.transform(v),
                Some(spec::Data::Flipv(ref v)) => self.transform(v),
                Some(spec::Data::Resize(ref v)) => self.transform(v),
                Some(spec::Data::Watermark(ref v)) => self.transform(v),
                // 对于目前不认识的 spec，不做任何处理
                _ => {}
            }
        }
    }

    fn generate(self, format: ImageOutputFormat) -> Vec<u8> {
        image_to_buf(self.0, format)
    }
}

impl SpecTransform<&Cron> for Photon {
    fn transform(&mut self, op: &Contrast) {
        effects::adjust_contrast(&mut self.0, op.contrast);
    }
}

impl SpecTransform(&Flipv) for Photon {
    fn transform(&mut self, _op: &Flipv) {
        transform::flipv(&mut self.0);
    }
}

impl SpecTransform(&Fliph) for Photon {
    fn transform(&mut self, _op: &Fliph) {
        transform::fliph(&mut self.0);
    }
}

impl SpecTransform(&Resize) for Photon {
    fn transform(&mut self, op: &Resize) {
        let img = match resize::ResizeType::from_i32(op.rtype).unwrap() {
            resize::ResizeType::Normal => transform::resize(
                &mut self.0,
                op.width,
                op.height,
                resize::SamplingFilter::from_i32(op.filter).unwrap().into(),
            ),
            resize::ResizeType::SeamCarve => {
                transform::seam_carve(&mut self.0, op.width, op.height)
            }
        };
        self.0 = img
    }
}

impl SpecTransform<&Watermark> for Photon {
    fn transform(&mut self, op: &Watermark) {
        multiple::watermark(&mut self.0, &WATERMARK, op.x, op.y);
    }   
}

fn image_to_buf(img: PhotonImage, format: ImageOutputFormat) -> Vec<u8> {
    let raw_pixels = img.raw_pixels();
    let width = img.width();
    let height = img.height();

    let image_buffer = ImageBuffer::from_raw(width, height, raw_pixels).unwrap();
    let dynamic_image = DynamicImage::ImageRgba8(image_buffer);
    let mut buffer = Vec::with_capacity(32768);
    dynamic_image.write_to(&mut buffer, format).unwrap();
    buffer
}
