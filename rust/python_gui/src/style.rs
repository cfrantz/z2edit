use anyhow::Result;
use imgui::{Direction, StyleColor};
use imgui_sys as sys;
use indexmap::IndexMap;
use pyo3::prelude::*;
use serde::{Deserialize, Serialize};

/// A cardinal direction
#[pyclass(eq, eq_int)]
#[repr(i32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub enum JsonDirection {
    None = sys::ImGuiDir_None,
    Left = sys::ImGuiDir_Left,
    Right = sys::ImGuiDir_Right,
    Up = sys::ImGuiDir_Up,
    Down = sys::ImGuiDir_Down,
}

impl From<Direction> for JsonDirection {
    fn from(x: Direction) -> Self {
        match x {
            Direction::None => JsonDirection::None,
            Direction::Left => JsonDirection::Left,
            Direction::Right => JsonDirection::Right,
            Direction::Up => JsonDirection::Up,
            Direction::Down => JsonDirection::Down,
        }
    }
}

impl From<JsonDirection> for Direction {
    fn from(x: JsonDirection) -> Self {
        match x {
            JsonDirection::None => Direction::None,
            JsonDirection::Left => Direction::Left,
            JsonDirection::Right => Direction::Right,
            JsonDirection::Up => Direction::Up,
            JsonDirection::Down => Direction::Down,
        }
    }
}

/// User interface style/colors
#[pyclass]
#[pyo3(get_all, set_all)]
#[repr(C)]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JsonStyle {
    /// Global alpha applies to everything
    pub alpha: f32,
    /// Additional alpha multiplier applied to disabled elements. Multiplies over current value of [`Style::alpha`].
    pub disabled_alpha: f32,
    /// Padding within a window
    pub window_padding: [f32; 2],
    /// Rounding radius of window corners.
    ///
    /// Set to 0.0 to have rectangular windows.
    /// Large values tend to lead to a variety of artifacts and are not recommended.
    pub window_rounding: f32,
    /// Thickness of border around windows.
    ///
    /// Generally set to 0.0 or 1.0 (other values are not well tested and cost more CPU/GPU).
    pub window_border_size: f32,
    /// Minimum window size
    pub window_min_size: [f32; 2],
    /// Alignment for title bar text.
    ///
    /// Defaults to [0.5, 0.5] for left-aligned, vertically centered.
    pub window_title_align: [f32; 2],
    /// Side of the collapsing/docking button in the title bar (left/right).
    ///
    /// Defaults to [`Direction::Left`].
    pub window_menu_button_position: JsonDirection,
    /// Rounding radius of child window corners.
    ///
    /// Set to 0.0 to have rectangular child windows.
    pub child_rounding: f32,
    /// Thickness of border around child windows.
    ///
    /// Generally set to 0.0 or 1.0 (other values are not well tested and cost more CPU/GPU).
    pub child_border_size: f32,
    /// Rounding radius of popup window corners.
    ///
    /// Note that tooltip windows use `window_rounding` instead.
    pub popup_rounding: f32,
    /// Thickness of border around popup/tooltip windows.
    ///
    /// Generally set to 0.0 or 1.0 (other values are not well tested and cost more CPU/GPU).
    pub popup_border_size: f32,
    /// Padding within a framed rectangle (used by most widgets)
    pub frame_padding: [f32; 2],
    /// Rounding radius of frame corners (used by most widgets).
    ///
    /// Set to 0.0 to have rectangular frames.
    pub frame_rounding: f32,
    /// Thickness of border around frames.
    ///
    /// Generally set to 0.0 or 1.0 (other values are not well tested and cost more CPU/GPU).
    pub frame_border_size: f32,
    /// Horizontal and vertical spacing between widgets/lines
    pub item_spacing: [f32; 2],
    /// Horizontal and vertical spacing between elements of a composed widget (e.g. a slider and
    /// its label)
    pub item_inner_spacing: [f32; 2],
    /// Padding within a table cell.
    pub cell_padding: [f32; 2],
    /// Expand reactive bounding box for touch-based system where touch position is not accurate
    /// enough.
    ///
    /// Unfortunately we don't sort widgets so priority on overlap will always be given to the
    /// first widget, so don't grow this too much.
    pub touch_extra_padding: [f32; 2],
    /// Horizontal indentation when e.g. entering a tree node.
    ///
    /// Generally equal to (font size + horizontal frame padding * 2).
    pub indent_spacing: f32,
    /// Minimum horizontal spacing between two columns
    pub columns_min_spacing: f32,
    /// Width of the vertical scrollbar, height of the horizontal scrollbar
    pub scrollbar_size: f32,
    /// Rounding radius of scrollbar grab corners
    pub scrollbar_rounding: f32,
    /// Minimum width/height of a grab box for slider/scrollbar
    pub grab_min_size: f32,
    /// Rounding radius of grab corners.
    ///
    /// Set to 0.0 to have rectangular slider grabs.
    pub grab_rounding: f32,
    /// The size in pixels of the dead-zone around zero on logarithmic sliders that cross zero
    pub log_slider_deadzone: f32,
    /// Rounding radius of upper corners of tabs.
    ///
    /// Set to 0.0 to have rectangular tabs.
    pub tab_rounding: f32,
    /// Thickness of border around tabs
    pub tab_border_size: f32,
    /// Minimum width for close button to appear on an unselected tab when hovered.
    ///
    /// `= 0.0`: always show when hovering
    /// `= f32::MAX`: never show close button unless selected
    pub tab_min_width_for_close_button: f32,

    /// Thickness of tab-bar separator, which takes on the tab active color to denote focus.
    pub tab_bar_border_size: f32,

    /// Thickness of tab-bar overline, which highlights the selected tab-bar.
    pub tab_bar_overline_size: f32,

    /// Angle of angled headers (supported values range from -50.0f degrees to +50.0f degrees).
    pub table_angled_headers_angle: f32,

    /// Alignment of angled headers within the cell
    pub table_angled_headers_text_align: [f32; 2],

    /// Side of the color buttonton pubin color editor widgets (left/right).
    ///
    /// Defaults to [`Direction::Right`].
    pub color_button_position: JsonDirection,
    /// Alignment of button text when button is larger than text.
    ///
    /// Defaults to [0.5, 0.5] (centered).
    pub button_text_align: [f32; 2],
    /// Alignment of selectable text when selectable is larger than text.
    ///
    /// Defaults to [0.5, 0.5] (top-left aligned).
    pub selectable_text_align: [f32; 2],
    /// Thickkness of border in [`Ui::separator_with_text`](crate::Ui::separator_with_text)
    pub separator_text_border_size: f32,
    /// Alignment of text within the separator. Defaults to `[0.0, 0.5]` (left aligned, center).
    pub separator_text_align: [f32; 2],
    /// Horizontal offset of text from each edge of the separator + spacing on other axis.
    /// Generally small values. .y is recommended to be == [`StyleVar::FramePadding`].y.
    pub separator_text_padding: [f32; 2],

    /// Window positions are clamped to be visible within the display area or monitors by at least
    /// this amount.
    ///
    /// Only applies to regular windows.
    pub display_window_padding: [f32; 2],
    /// If you cannot see the edges of your screen (e.g. on a TV), increase the safe area padding.
    ///
    /// Also applies to popups/tooltips in addition to regular windows.
    pub display_safe_area_padding: [f32; 2],

    /// Thickness of resizing border between docked windows
    pub docking_separator_size: f32,

    /// Scale software-rendered mouse cursor.
    ///
    /// May be removed later.
    pub mouse_cursor_scale: f32,
    /// Enable anti-aliased lines/borders.
    ///
    /// Disable if you are really tight on CPU/GPU. Latched at the beginning of the frame.
    pub anti_aliased_lines: bool,
    /// Enable anti-aliased lines/borders using textures where possible.
    ///
    /// Require back-end to render with bilinear filtering. Latched at the beginning of the frame.
    pub anti_aliased_lines_use_tex: bool,
    /// Enable anti-aliased edges around filled shapes (rounded recatngles, circles, etc.).
    ///
    /// Disable if you are really tight on CPU/GPU. Latched at the beginning of the frame.
    pub anti_aliased_fill: bool,
    /// Tessellation tolerance when using path_bezier_curve_to without a specific number of
    /// segments.
    ///
    /// Decrease for highly tessellated curves (higher quality, more polygons), increase to reduce
    /// quality.
    pub curve_tessellation_tol: f32,
    /// Maximum error (in pixels) allowed when drawing circles or rounded corner rectangles with no
    /// explicit segment count specified.
    ///
    /// Decrease for higher quality but more geometry.
    pub circle_tesselation_max_error: f32,

    /// Style colors.
    pub colors: IndexMap<String, [f32; 4]>,

    /// Delay on hover before
    /// [`Ui::is_item_hovered_with_flags`](crate::Ui::is_item_hovered_with_flags) + [`HoveredFlags::STATIONARY`] returns true
    pub hover_stationary_delay: f32,

    /// Delay on hover before
    /// [`Ui::is_item_hovered_with_flags`](crate::Ui::is_item_hovered_with_flags) + [`HoveredFlags::DELAY_SHORT`] returns true
    pub hover_delay_short: f32,

    /// Delay on hover before
    /// [`Ui::is_item_hovered_with_flags`](crate::Ui::is_item_hovered_with_flags) + [`HoveredFlags::DELAY_NORMAL`] returns true
    pub hover_delay_normal: f32,

    /// Default flags when using [`HoveredFlags::FOR_TOOLTIP`] or [`Ui::begin_tooltip`](crate::Ui::begin_tooltip)
    /// or [`Ui::tooltip_text`](crate::Ui::tooltip_text) while using mouse.
    pub hover_flags_for_tooltip_mouse: u32,
    /// Default flags when using [`HoveredFlags::FOR_TOOLTIP`] or [`Ui::begin_tooltip`](crate::Ui::begin_tooltip)
    /// or [`Ui::tooltip_text`](crate::Ui::tooltip_text) while using keyboard/gamepad.
    pub hover_flags_for_tooltip_nav: u32,
}

impl Default for JsonStyle {
    fn default() -> Self {
        JsonStyle::from(&imgui::Style::default())
    }
}

#[pymethods]
impl JsonStyle {
    #[staticmethod]
    pub fn from_json(s: &str) -> Result<Self> {
        Ok(serde_json::from_str::<Self>(s)?)
    }

    pub fn to_json(&self) -> Result<String> {
        Ok(serde_json::to_string_pretty(self)?)
    }
}

impl From<&imgui::Style> for JsonStyle {
    fn from(style: &imgui::Style) -> Self {
        let mut colors = IndexMap::new();
        for (i, color) in style.colors.iter().enumerate() {
            let sc = StyleColor::VARIANTS[i];
            colors.insert(sc.name().into(), *color);
        }
        Self {
            alpha: style.alpha,
            disabled_alpha: style.disabled_alpha,
            window_padding: style.window_padding,
            window_rounding: style.window_rounding,
            window_border_size: style.window_border_size,
            window_min_size: style.window_min_size,
            window_title_align: style.window_title_align,
            window_menu_button_position: style.window_menu_button_position.into(),
            child_rounding: style.child_rounding,
            child_border_size: style.child_border_size,
            popup_rounding: style.popup_rounding,
            popup_border_size: style.popup_border_size,
            frame_padding: style.frame_padding,
            frame_rounding: style.frame_rounding,
            frame_border_size: style.frame_border_size,
            item_spacing: style.item_spacing,
            item_inner_spacing: style.item_inner_spacing,
            cell_padding: style.cell_padding,
            touch_extra_padding: style.touch_extra_padding,
            indent_spacing: style.indent_spacing,
            columns_min_spacing: style.columns_min_spacing,
            scrollbar_size: style.scrollbar_size,
            scrollbar_rounding: style.scrollbar_rounding,
            grab_min_size: style.grab_min_size,
            grab_rounding: style.grab_rounding,
            log_slider_deadzone: style.log_slider_deadzone,
            tab_rounding: style.tab_rounding,
            tab_border_size: style.tab_border_size,
            tab_min_width_for_close_button: style.tab_min_width_for_close_button,
            tab_bar_border_size: style.tab_bar_border_size,
            tab_bar_overline_size: style.tab_bar_overline_size,
            table_angled_headers_angle: style.table_angled_headers_angle,
            table_angled_headers_text_align: style.table_angled_headers_text_align,
            color_button_position: style.color_button_position.into(),
            button_text_align: style.button_text_align,
            selectable_text_align: style.selectable_text_align,
            separator_text_border_size: style.separator_text_border_size,
            separator_text_align: style.separator_text_align,
            separator_text_padding: style.separator_text_padding,
            display_window_padding: style.display_window_padding,
            display_safe_area_padding: style.display_safe_area_padding,
            docking_separator_size: style.docking_separator_size,
            mouse_cursor_scale: style.mouse_cursor_scale,
            anti_aliased_lines: style.anti_aliased_lines,
            anti_aliased_lines_use_tex: style.anti_aliased_lines_use_tex,
            anti_aliased_fill: style.anti_aliased_fill,
            curve_tessellation_tol: style.curve_tessellation_tol,
            circle_tesselation_max_error: style.circle_tesselation_max_error,
            colors,
            hover_stationary_delay: style.hover_stationary_delay,
            hover_delay_short: style.hover_delay_short,
            hover_delay_normal: style.hover_delay_normal,
            hover_flags_for_tooltip_mouse: style.hover_flags_for_tooltip_mouse.bits(),
            hover_flags_for_tooltip_nav: style.hover_flags_for_tooltip_nav.bits(),
        }
    }
}

impl From<&JsonStyle> for imgui::Style {
    fn from(style: &JsonStyle) -> Self {
        let mut names = IndexMap::new();
        for (i, sc) in StyleColor::VARIANTS.iter().enumerate() {
            names.insert(sc.name(), i);
        }
        let mut colors = StyleColor::dark_colors();
        for (name, color) in style.colors.iter() {
            if let Some(i) = names.get(name.as_str()) {
                colors[*i] = *color;
            } else {
                // error
            }
        }
        Self {
            alpha: style.alpha,
            disabled_alpha: style.disabled_alpha,
            window_padding: style.window_padding,
            window_rounding: style.window_rounding,
            window_border_size: style.window_border_size,
            window_min_size: style.window_min_size,
            window_title_align: style.window_title_align,
            window_menu_button_position: style.window_menu_button_position.into(),
            child_rounding: style.child_rounding,
            child_border_size: style.child_border_size,
            popup_rounding: style.popup_rounding,
            popup_border_size: style.popup_border_size,
            frame_padding: style.frame_padding,
            frame_rounding: style.frame_rounding,
            frame_border_size: style.frame_border_size,
            item_spacing: style.item_spacing,
            item_inner_spacing: style.item_inner_spacing,
            cell_padding: style.cell_padding,
            touch_extra_padding: style.touch_extra_padding,
            indent_spacing: style.indent_spacing,
            columns_min_spacing: style.columns_min_spacing,
            scrollbar_size: style.scrollbar_size,
            scrollbar_rounding: style.scrollbar_rounding,
            grab_min_size: style.grab_min_size,
            grab_rounding: style.grab_rounding,
            log_slider_deadzone: style.log_slider_deadzone,
            tab_rounding: style.tab_rounding,
            tab_border_size: style.tab_border_size,
            tab_min_width_for_close_button: style.tab_min_width_for_close_button,
            tab_bar_border_size: style.tab_bar_border_size,
            tab_bar_overline_size: style.tab_bar_overline_size,
            table_angled_headers_angle: style.table_angled_headers_angle,
            table_angled_headers_text_align: style.table_angled_headers_text_align,
            color_button_position: style.color_button_position.into(),
            button_text_align: style.button_text_align,
            selectable_text_align: style.selectable_text_align,
            separator_text_border_size: style.separator_text_border_size,
            separator_text_align: style.separator_text_align,
            separator_text_padding: style.separator_text_padding,
            display_window_padding: style.display_window_padding,
            display_safe_area_padding: style.display_safe_area_padding,
            docking_separator_size: style.docking_separator_size,
            mouse_cursor_scale: style.mouse_cursor_scale,
            anti_aliased_lines: style.anti_aliased_lines,
            anti_aliased_lines_use_tex: style.anti_aliased_lines_use_tex,
            anti_aliased_fill: style.anti_aliased_fill,
            curve_tessellation_tol: style.curve_tessellation_tol,
            circle_tesselation_max_error: style.circle_tesselation_max_error,
            colors,
            hover_stationary_delay: style.hover_stationary_delay,
            hover_delay_short: style.hover_delay_short,
            hover_delay_normal: style.hover_delay_normal,
            hover_flags_for_tooltip_mouse: unsafe {
                std::mem::transmute(style.hover_flags_for_tooltip_mouse)
            },
            hover_flags_for_tooltip_nav: unsafe {
                std::mem::transmute(style.hover_flags_for_tooltip_nav)
            },
        }
    }
}
