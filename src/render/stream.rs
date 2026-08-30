use crate::types::{Color, VectorShape};

#[derive(Clone, Debug)]
pub enum ApiPayload {
    ChartCandle {
        x: f32,
        open: f32,
        high: f32,
        low: f32,
        close: f32,
        width: f32,
    },
    UiBox {
        x: f32,
        y: f32,
        w: f32,
        h: f32,
        color: Color,
    },
    RawText {
        text: String,
        x: f32,
        y: f32,
        size: f32,
        color: Color,
    },
}

pub struct DataStreamAdapter;

impl DataStreamAdapter {
    pub fn parse_stream_to_shapes(payloads: &[ApiPayload]) -> Vec<VectorShape> {
        payloads
            .iter()
            .map(|payload| match payload {
                ApiPayload::ChartCandle {
                    x,
                    open,
                    high,
                    low,
                    close,
                    width,
                } => VectorShape::CandleStick {
                    x: *x,
                    open: *open,
                    high: *high,
                    low: *low,
                    close: *close,
                    width: *width,
                },
                ApiPayload::UiBox { x, y, w, h, color } => VectorShape::Rectangle {
                    x: *x,
                    y: *y,
                    w: *w,
                    h: *h,
                    color: *color,
                    border_radius: 0.0,
                },
                ApiPayload::RawText {
                    text,
                    x,
                    y,
                    size,
                    color,
                } => VectorShape::Text {
                    x: *x,
                    y: *y,
                    body: text.clone(),
                    size: *size,
                    color: *color,
                },
            })
            .collect()
    }
}