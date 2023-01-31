use cesrox_core::error::CesrResult;
use cesrox_core::groups::CesrGroup;
pub use cesrox_core::groups::Frame;

/*
    Frame
*/
pub struct FrameFromStreamResult {
    pub rest: Vec<u8>,
    pub message: Frame
}

pub fn frame_create(value: Vec<CesrGroup>) -> Frame {
    Frame::new(value)
}

pub fn frame_to_str(frame: &Frame) -> String {
    frame.to_str()
}

pub fn frame_from_str(str: &str) -> CesrResult<Frame> {
    Frame::from_str(str)
}

pub fn frame_to_bytes(frame: &Frame) -> Vec<u8> {
    frame.to_bytes()
}

pub fn frame_from_bytes(bytes: &[u8]) -> CesrResult<Frame> {
    Frame::from_bytes(bytes)
}

pub fn frame_from_stream_bytes(str: &[u8]) -> CesrResult<FrameFromStreamResult> {
    let (res, message) = Frame::from_stream_bytes(str)?;
    Ok(FrameFromStreamResult {
        rest: res.to_vec(),
        message
    })
}