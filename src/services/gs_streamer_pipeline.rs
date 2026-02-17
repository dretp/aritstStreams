use anyhow::Result;
use gstreamer as gst;
use gstreamer::prelude::*;
use crate::models::{VideoJob, VideoInput};
pub struct GStreamerPipelineService;



impl GStreamerPipelineService {
    pub fn run(job: &mut VideoJob) -> Result<()> {
        gst::init()?;

        job.mark_processing();

        let src = match &job.input {
            VideoInput::Upload { path } => {
                format!("filesrc location={}", path)
            }
            VideoInput::Url { url } => {
                format!("urisourcebin uri={}", url)
            }
            VideoInput::Stream { stream_id } => {
                format!("rtmpsrc location={}", stream_id)
            }
        };

        let pipeline_str = format!(
            "{src} !
             decodebin !
             videoconvert !
             x264enc !
             hlssink location={}/segment%05d.ts playlist-location={}/index.m3u8",
            job.output_dir,
            job.output_dir
        );

        let pipeline = gst::parse::launch(&pipeline_str)?
            .downcast::<gst::Pipeline>()
            .unwrap();

        pipeline.set_state(gst::State::Playing)?;

        let bus = pipeline.bus().unwrap();
        for msg in bus.iter_timed(gst::ClockTime::NONE) {
            match msg.view() {
                gst::MessageView::Eos(..) => {
                    job.mark_completed();
                    break;
                }
                gst::MessageView::Error(err) => {
                    job.mark_failed(err.error().to_string());
                    break;
                }
                _ => {}
            }
        }

        pipeline.set_state(gst::State::Null)?;
        Ok(())
    }
}
