use eframe::egui::{self, TextureWrapMode};
use log::{trace, warn};
use rusty_spine::atlas::{AtlasFilter, AtlasFormat, AtlasWrap};
use wgpu::TextureFormat;

use crate::texture::SpineTexture;

pub trait Operator: std::fmt::Debug {
    fn render(&mut self);
}

fn set_texture_cb() {
    rusty_spine::extension::set_create_texture_cb(|atlas_page: &mut rusty_spine::atlas::AtlasPage, path: &str| {
        trace!("texture callback for {:?}", path);
        fn convert_filter(filter: AtlasFilter) -> egui::TextureFilter {
            match filter {
                AtlasFilter::Linear => egui::TextureFilter::Linear,
                AtlasFilter::Nearest => egui::TextureFilter::Nearest,
                filter => {
                    warn!("Unsupported texture filter mode: {filter:?}");
                    egui::TextureFilter::Linear
                }
            }
        }
        fn convert_wrap(wrap: AtlasWrap) -> TextureWrapMode {
            match wrap {
                AtlasWrap::ClampToEdge => TextureWrapMode::ClampToEdge,
                AtlasWrap::MirroredRepeat => TextureWrapMode::MirroredRepeat,
                AtlasWrap::Repeat => TextureWrapMode::Repeat,
                wrap => {
                    warn!("Unsupported texture wrap mode: {wrap:?}");
                    TextureWrapMode::ClampToEdge
                }
            }
        }

        fn convert_format(format: AtlasFormat) -> TextureFormat {
            match format {
                AtlasFormat::RGBA8888 => TextureFormat::Rgba8Unorm,
                AtlasFormat::RGB888 => TextureFormat::Rgba8Unorm,
                _ => {
                    warn!("Unsupported texture format: {:?}", format);
                    TextureFormat::Rgba8Unorm
                }
            }
        }

        atlas_page
            .renderer_object()
            .set(SpineTexture::Pending {
                path: path.to_owned(),
                min_filter: convert_filter(atlas_page.min_filter()),
                mag_filter: convert_filter(atlas_page.mag_filter()),
                x_wrap: convert_wrap(atlas_page.u_wrap()),
                y_wrap: convert_wrap(atlas_page.v_wrap()),
                format: convert_format(atlas_page.format()),
            });
    });
    rusty_spine::extension::set_dispose_texture_cb(|atlas_page: &mut rusty_spine::atlas::AtlasPage| unsafe {
        atlas_page.renderer_object().dispose::<SpineTexture>();
    });
}
