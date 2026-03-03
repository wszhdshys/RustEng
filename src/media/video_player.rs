use ffmpeg_next::Error;
use ffmpeg_next::software::scaling;
use crate::config::ENGINE_CONFIG;

pub struct VideoPlayer {
    frames: Vec<slint::Image>,
    fps: u32,
}

impl VideoPlayer {
    pub fn new(name: &str) -> Result<VideoPlayer, Error> {
        ffmpeg_next::init().unwrap();
        let mut ictx = ffmpeg_next::format::input(&format!("{}{}.wmv", ENGINE_CONFIG.video_path(), name))?;

        let input = ictx
            .streams()
            .best(ffmpeg_next::media::Type::Video)
            .ok_or("no video stream")?;

        let video_stream_index = input.index();

        let audio_stream = ictx
            .streams()
            .best(ffmpeg_next::media::Type::Audio);
        let audio_stream_index = audio_stream.map(|s| s.index());

        let video_stream = ictx.stream(video_stream_index).unwrap();
        let mut video_decoder = ffmpeg_next::codec::context::Context::from_parameters(
            video_stream.parameters(),
        )?
            .decoder()
            .video()?;

        let mut scaler = scaling::Context::get(
            video_decoder.format(),
            video_decoder.width(),
            video_decoder.height(),
            ffmpeg_next::format::Pixel::RGBA,
            video_decoder.width(),
            video_decoder.height(),
            scaling::flag::Flags::BILINEAR,
        )?;

        Ok(())
    }
}