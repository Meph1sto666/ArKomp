use eframe::egui::{TextureFilter, TextureHandle, TextureWrapMode};
use wgpu::TextureFormat;

pub enum SpineTexture {
	Pending {
		path: String,
		min_filter: TextureFilter,
		mag_filter: TextureFilter,
		x_wrap: TextureWrapMode,
		y_wrap: TextureWrapMode,
		format: TextureFormat,
	},
	Loaded(TextureHandle)
}