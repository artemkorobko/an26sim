use crate::common::percent::Percent;

#[macro_export]
macro_rules! label {
    ($text:literal, $parent:expr, $rect:expr) => {{
        let label_name = super::api::create_label($parent, $rect, $text)?;
        super::api::make_widget_lit(label_name);
        let label_value = super::api::create_label($parent, &$rect.to_value_line(), "")?;
        super::api::make_widget_lit(label_value);
        label_value
    }};
}

pub fn format_percent(value: f32, min: f32, max: f32) -> String {
    let percent = value.percent_from_value(min, max);
    format!("{:.2}\u{0025}", percent)
}

pub fn format_degree(value: f32) -> String {
    format!("{:.2}\u{00b0}", value)
}
