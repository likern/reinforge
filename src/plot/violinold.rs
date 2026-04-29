use plotly::Plot;
use plotly::plot::Trace;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct ViolinTrace {
    #[serde(rename = "type")]
    trace_type: &'static str,

    x: Vec<String>,
    y: Vec<f64>,

    name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    box_visible: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    meanline_visible: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    points: Option<&'static str>,
}

impl Trace for ViolinTrace {
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}
