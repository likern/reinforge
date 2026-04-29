use std::collections::BTreeMap;

use plotly::Trace;
use plotly::common::{Orientation, Visible};
use serde::Serialize;
use serde_json::{Value, json};

use super::enums::{
    QuartileMethod, ViolinHoverOn, ViolinPoints, ViolinScaleMode, ViolinSide, ViolinSpanMode,
};

#[derive(Clone, Debug, serde::Serialize)]
pub struct Violin {
    #[serde(flatten)]
    props: BTreeMap<String, Value>,
}

impl Violin {
    fn to_value<T: Serialize>(value: T) -> Value {
        serde_json::to_value(value).expect("failed to serialize violin attribute")
    }

    fn merge_values(dst: &mut Value, src: Value) {
        match (dst, src) {
            (Value::Object(dst_obj), Value::Object(src_obj)) => {
                for (k, v) in src_obj {
                    match dst_obj.get_mut(&k) {
                        Some(existing) => Self::merge_values(existing, v),
                        None => {
                            dst_obj.insert(k, v);
                        }
                    }
                }
            }
            (dst_slot, src_val) => {
                *dst_slot = src_val;
            }
        }
    }

    fn set_attr(mut self: Box<Self>, key: impl Into<String>, value: impl Serialize) -> Box<Self> {
        self.props.insert(key.into(), Self::to_value(value));
        self
    }

    fn merge_attr(mut self: Box<Self>, key: impl Into<String>, patch: impl Serialize) -> Box<Self> {
        let key = key.into();
        let patch_value = Self::to_value(patch);

        match self.props.get_mut(&key) {
            Some(existing) => Self::merge_values(existing, patch_value),
            None => {
                self.props.insert(key, patch_value);
            }
        }

        self
    }

    pub fn new<Y: Serialize>(y: Vec<Y>) -> Box<Self> {
        let mut props = BTreeMap::new();
        props.insert("type".into(), json!("violin"));
        props.insert("y".into(), Self::to_value(y));

        Box::new(Self { props })
    }

    pub fn new_xy<X: Serialize, Y: Serialize>(x: Vec<X>, y: Vec<Y>) -> Box<Self> {
        let mut props = BTreeMap::new();
        props.insert("type".into(), json!("violin"));
        props.insert("x".into(), Self::to_value(x));
        props.insert("y".into(), Self::to_value(y));

        Box::new(Self { props })
    }

    pub fn raw_attr(self: Box<Self>, key: impl Into<String>, value: impl Serialize) -> Box<Self> {
        self.set_attr(key, value)
    }

    pub fn name(self: Box<Self>, value: impl Into<String>) -> Box<Self> {
        self.set_attr("name", value.into())
    }

    pub fn visible(self: Box<Self>, value: Visible) -> Box<Self> {
        self.set_attr("visible", value)
    }

    pub fn show_legend(self: Box<Self>, value: bool) -> Box<Self> {
        self.set_attr("showlegend", value)
    }

    pub fn legend_group(self: Box<Self>, value: impl Into<String>) -> Box<Self> {
        self.set_attr("legendgroup", value.into())
    }

    pub fn opacity(self: Box<Self>, value: f64) -> Box<Self> {
        self.set_attr("opacity", value)
    }

    pub fn width(self: Box<Self>, value: f64) -> Box<Self> {
        self.set_attr("width", value)
    }

    pub fn orientation(self: Box<Self>, value: Orientation) -> Box<Self> {
        self.set_attr("orientation", value)
    }

    pub fn x0<T: Serialize>(self: Box<Self>, value: T) -> Box<Self> {
        self.set_attr("x0", value)
    }

    pub fn y0<T: Serialize>(self: Box<Self>, value: T) -> Box<Self> {
        self.set_attr("y0", value)
    }

    pub fn ids<T: Serialize>(self: Box<Self>, value: Vec<T>) -> Box<Self> {
        self.set_attr("ids", value)
    }

    pub fn hover_template(self: Box<Self>, value: impl Into<String>) -> Box<Self> {
        self.set_attr("hovertemplate", value.into())
    }

    pub fn hover_info(self: Box<Self>, value: impl Into<String>) -> Box<Self> {
        self.set_attr("hoverinfo", value.into())
    }

    pub fn text<T: Serialize>(self: Box<Self>, value: T) -> Box<Self> {
        self.set_attr("text", value)
    }

    pub fn hover_text<T: Serialize>(self: Box<Self>, value: T) -> Box<Self> {
        self.set_attr("hovertext", value)
    }

    pub fn custom_data<T: Serialize>(self: Box<Self>, value: T) -> Box<Self> {
        self.set_attr("customdata", value)
    }

    pub fn meta<T: Serialize>(self: Box<Self>, value: T) -> Box<Self> {
        self.set_attr("meta", value)
    }

    pub fn x_axis(self: Box<Self>, value: impl Into<String>) -> Box<Self> {
        self.set_attr("xaxis", value.into())
    }

    pub fn y_axis(self: Box<Self>, value: impl Into<String>) -> Box<Self> {
        self.set_attr("yaxis", value.into())
    }

    pub fn alignment_group(self: Box<Self>, value: impl Into<String>) -> Box<Self> {
        self.set_attr("alignmentgroup", value.into())
    }

    pub fn offset_group(self: Box<Self>, value: impl Into<String>) -> Box<Self> {
        self.set_attr("offsetgroup", value.into())
    }

    pub fn bandwidth(self: Box<Self>, value: f64) -> Box<Self> {
        self.set_attr("bandwidth", value)
    }

    pub fn fill_color(self: Box<Self>, value: impl Into<String>) -> Box<Self> {
        self.set_attr("fillcolor", value.into())
    }

    pub fn line_color(self: Box<Self>, value: impl Into<String>) -> Box<Self> {
        self.merge_attr("line", json!({ "color": value.into() }))
    }

    pub fn line_width(self: Box<Self>, value: f64) -> Box<Self> {
        self.merge_attr("line", json!({ "width": value }))
    }

    pub fn marker_color(self: Box<Self>, value: impl Into<String>) -> Box<Self> {
        self.merge_attr("marker", json!({ "color": value.into() }))
    }

    pub fn marker_size(self: Box<Self>, value: f64) -> Box<Self> {
        self.merge_attr("marker", json!({ "size": value }))
    }

    pub fn marker_opacity(self: Box<Self>, value: f64) -> Box<Self> {
        self.merge_attr("marker", json!({ "opacity": value }))
    }

    pub fn box_visible(self: Box<Self>, value: bool) -> Box<Self> {
        self.merge_attr("box", json!({ "visible": value }))
    }

    pub fn box_fill_color(self: Box<Self>, value: impl Into<String>) -> Box<Self> {
        self.merge_attr("box", json!({ "fillcolor": value.into() }))
    }

    pub fn box_line_color(self: Box<Self>, value: impl Into<String>) -> Box<Self> {
        self.merge_attr("box", json!({ "line": { "color": value.into() } }))
    }

    pub fn box_line_width(self: Box<Self>, value: f64) -> Box<Self> {
        self.merge_attr("box", json!({ "line": { "width": value } }))
    }

    pub fn box_width(self: Box<Self>, value: f64) -> Box<Self> {
        self.merge_attr("box", json!({ "width": value }))
    }

    pub fn quartile_method(self: Box<Self>, value: QuartileMethod) -> Box<Self> {
        self.set_attr("quartilemethod", value)
    }

    pub fn selected_points<T: Serialize>(self: Box<Self>, value: T) -> Box<Self> {
        self.set_attr("selectedpoints", value)
    }

    pub fn hover_on(self: Box<Self>, value: ViolinHoverOn) -> Box<Self> {
        self.set_attr("hoveron", value)
    }

    pub fn point_pos(self: Box<Self>, value: f64) -> Box<Self> {
        self.set_attr("pointpos", value)
    }

    pub fn jitter(self: Box<Self>, value: f64) -> Box<Self> {
        self.set_attr("jitter", value)
    }

    pub fn meanline_visible(self: Box<Self>, value: bool) -> Box<Self> {
        self.merge_attr("meanline", json!({ "visible": value }))
    }

    pub fn meanline_color(self: Box<Self>, value: impl Into<String>) -> Box<Self> {
        self.merge_attr("meanline", json!({ "color": value.into() }))
    }

    pub fn meanline_width(self: Box<Self>, value: f64) -> Box<Self> {
        self.merge_attr("meanline", json!({ "width": value }))
    }

    pub fn points(self: Box<Self>, value: ViolinPoints) -> Box<Self> {
        self.set_attr("points", value)
    }

    pub fn scale_group(self: Box<Self>, value: impl Into<String>) -> Box<Self> {
        self.set_attr("scalegroup", value.into())
    }

    pub fn scale_mode(self: Box<Self>, value: ViolinScaleMode) -> Box<Self> {
        self.set_attr("scalemode", value)
    }

    pub fn side(self: Box<Self>, value: ViolinSide) -> Box<Self> {
        self.set_attr("side", value)
    }

    pub fn span(self: Box<Self>, value: Vec<f64>) -> Box<Self> {
        self.set_attr("span", value)
    }

    pub fn span_mode(self: Box<Self>, value: ViolinSpanMode) -> Box<Self> {
        self.set_attr("spanmode", value)
    }

    pub fn ui_revision<T: Serialize>(self: Box<Self>, value: T) -> Box<Self> {
        self.set_attr("uirevision", value)
    }
}

impl Trace for Violin {
    fn to_json(&self) -> String {
        serde_json::to_string(self).expect("failed to serialize Violin trace")
    }
}
