use std::{collections::BTreeMap, fs, path::Path};

use plotly::{ImageFormat, Plot, plotly_static::StaticExporterBuilder};
use serde::Serialize;
use serde_json::{Map, Value, json};

use super::enums::ViolinMode;

#[derive(Clone, Debug, Default, serde::Serialize)]
pub struct ViolinLayoutExtras {
    #[serde(flatten)]
    props: BTreeMap<String, Value>,
}

impl ViolinLayoutExtras {
    fn to_value<T: Serialize>(value: T) -> Value {
        serde_json::to_value(value).expect("failed to serialize violin layout extra")
    }

    pub fn new() -> Self {
        Self {
            props: BTreeMap::new(),
        }
    }

    pub fn raw_attr(mut self, key: impl Into<String>, value: impl Serialize) -> Self {
        self.props.insert(key.into(), Self::to_value(value));
        self
    }

    pub fn violin_mode(self, value: ViolinMode) -> Self {
        self.raw_attr("violinmode", value)
    }

    pub fn violin_gap(self, value: f64) -> Self {
        self.raw_attr("violingap", value)
    }

    pub fn violin_group_gap(self, value: f64) -> Self {
        self.raw_attr("violingroupgap", value)
    }

    pub fn as_map(&self) -> &BTreeMap<String, Value> {
        &self.props
    }
}

fn merge_values(dst: &mut Value, src: Value) {
    match (dst, src) {
        (Value::Object(dst_obj), Value::Object(src_obj)) => {
            for (k, v) in src_obj {
                match dst_obj.get_mut(&k) {
                    Some(existing) => merge_values(existing, v),
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

pub fn plot_spec_with_layout_extras(
    plot: &Plot,
    extras: &ViolinLayoutExtras,
) -> Result<Value, serde_json::Error> {
    let mut spec: Value = serde_json::from_str(&plot.to_json())?;

    if !spec.is_object() {
        spec = json!({});
    }

    let spec_obj = spec.as_object_mut().expect("plot spec must be an object");

    if !matches!(spec_obj.get("layout"), Some(Value::Object(_))) {
        spec_obj.insert("layout".into(), Value::Object(Map::new()));
    }

    let layout = spec_obj
        .get_mut("layout")
        .and_then(Value::as_object_mut)
        .expect("layout must be an object");

    for (k, v) in extras.as_map() {
        match layout.get_mut(k) {
            Some(existing) => merge_values(existing, v.clone()),
            None => {
                layout.insert(k.clone(), v.clone());
            }
        }
    }

    Ok(spec)
}

pub fn to_inline_html_with_layout_extras(
    plot: &Plot,
    extras: &ViolinLayoutExtras,
    div_id: Option<&str>,
    use_offline_js: bool,
) -> Result<String, serde_json::Error> {
    let div_id = div_id.unwrap_or("plot");
    let spec = plot_spec_with_layout_extras(plot, extras)?;
    let spec_json = serde_json::to_string(&spec)?;

    let js_sources = if use_offline_js {
        Plot::offline_js_sources()
    } else {
        Plot::online_cdn_js()
    };

    Ok(format!(
        r#"{js_sources}
<div id="{div_id}" style="width:100%;height:100%;"></div>
<script>
const spec = {spec_json};
const gd = document.getElementById("{div_id}");
Plotly.newPlot(gd, spec.data, spec.layout, spec.config).then(function() {{
    if (spec.frames && spec.frames.length) {{
        Plotly.addFrames(gd, spec.frames);
    }}
}});
</script>"#
    ))
}

pub fn to_standalone_html_with_layout_extras(
    plot: &Plot,
    extras: &ViolinLayoutExtras,
    div_id: Option<&str>,
    use_offline_js: bool,
) -> Result<String, serde_json::Error> {
    let inline = to_inline_html_with_layout_extras(plot, extras, div_id, use_offline_js)?;
    Ok(format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>Plotly violin</title>
</head>
<body>
{inline}
</body>
</html>"#
    ))
}

/// Экспорт figure с extra layout attrs в статическое изображение.
/// Подходит для SVG / PNG / WEBP / JPEG / PDF.
pub fn write_image_with_layout_extras<P: AsRef<Path>>(
    plot: &Plot,
    path: P,
    format: ImageFormat,
    width: usize,
    height: usize,
    scale: f64,
    extras: &ViolinLayoutExtras,
) -> Result<(), Box<dyn std::error::Error>> {
    let fig = plot_spec_with_layout_extras(plot, extras)?;

    let mut exporter = StaticExporterBuilder::default()
        .build()
        .map_err(|e| format!("Failed to create StaticExporter: {e}"))?;

    let result = exporter.write_fig(path.as_ref(), &fig, format, width, height, scale);

    exporter.close();
    result
}

/// Вернуть SVG как строку, уже с merged layout extras.
pub fn to_svg_with_layout_extras(
    plot: &Plot,
    width: usize,
    height: usize,
    scale: f64,
    extras: &ViolinLayoutExtras,
) -> Result<String, Box<dyn std::error::Error>> {
    let fig = plot_spec_with_layout_extras(plot, extras)?;

    let mut exporter = StaticExporterBuilder::default()
        .build()
        .map_err(|e| format!("Failed to create StaticExporter: {e}"))?;

    let result = exporter.write_to_string(&fig, ImageFormat::SVG, width, height, scale);

    exporter.close();
    result
}

pub fn write_html_with_layout_extras<P: AsRef<Path>>(
    plot: &Plot,
    path: P,
    extras: &ViolinLayoutExtras,
    use_offline_js: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let html = to_standalone_html_with_layout_extras(plot, extras, Some("plot"), use_offline_js)?;
    fs::write(path, html)?;
    Ok(())
}

/// Удобный helper: сразу записать SVG-файл.
pub fn write_svg_with_layout_extras<P: AsRef<Path>>(
    plot: &Plot,
    path: P,
    width: usize,
    height: usize,
    scale: f64,
    extras: &ViolinLayoutExtras,
) -> Result<(), Box<dyn std::error::Error>> {
    let svg = to_svg_with_layout_extras(plot, width, height, scale, extras)?;
    fs::write(path, svg)?;
    Ok(())
}
