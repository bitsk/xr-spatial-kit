// SPDX-License-Identifier: GPL-3.0-or-later
//! Local widget models and UKUI style data. Rendering and session ownership
//! belong to Shell; clients send these primitive descriptions through its SDK.
use anyhow::{Result, ensure};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::{collections::BTreeMap, path::Path};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Theme {
    pub schema: u32,
    pub id: String,
    pub font_families: Vec<String>,
    pub font_size: f32,
    pub corner_radius: f32,
    pub border_width: f32,
    pub colours: BTreeMap<String, [f32; 4]>,
    pub styles: BTreeMap<String, String>,
}
impl Default for Theme {
    fn default() -> Self {
        serde_json::from_str(include_str!("../styles/ukui.json")).expect("built-in UKUI tokens")
    }
}
impl Theme {
    pub fn load(path: &Path) -> Result<Self> {
        let theme: Self = serde_json::from_slice(&std::fs::read(path)?)?;
        ensure!(
            theme.schema == 1 && theme.font_size.is_finite() && theme.font_size > 0.0,
            "unsupported spatial style schema or text size"
        );
        ensure!(
            theme.corner_radius.is_finite()
                && theme.corner_radius >= 0.0
                && theme.border_width.is_finite()
                && theme.border_width >= 0.0,
            "invalid spatial geometry tokens"
        );
        ensure!(
            theme
                .colours
                .values()
                .flatten()
                .all(|v| v.is_finite() && (0.0..=1.0).contains(v)),
            "invalid spatial colour token"
        );
        ensure!(
            theme
                .styles
                .values()
                .all(|key| theme.colours.contains_key(key)),
            "unknown style colour"
        );
        Ok(theme)
    }
    pub fn colour(&self, style: &str) -> [f32; 4] {
        self.styles
            .get(style)
            .and_then(|k| self.colours.get(k))
            .copied()
            .or_else(|| self.colours.get(style).copied())
            .unwrap_or([0.96, 0.98, 1.0, 0.9])
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Kind {
    Card,
    Text,
    Icon,
    Button,
    Slider,
    SurfaceReference,
}

#[derive(Clone, Debug)]
pub struct Widget {
    pub id: String,
    pub parent_id: String,
    pub kind: Kind,
    /// Local logical pixels: top-left x/y followed by width/height.
    pub rect: [f32; 4],
    pub text: String,
    pub icon_id: String,
    pub action_id: String,
    pub style_id: String,
    pub enabled: bool,
    pub value: f32,
    pub min: f32,
    pub max: f32,
    pub surface: Option<(String, String)>,
}
impl Widget {
    fn new(id: &str, kind: Kind, rect: [f32; 4], style: &str) -> Self {
        Self {
            id: id.into(),
            parent_id: "root".into(),
            kind,
            rect,
            text: String::new(),
            icon_id: String::new(),
            action_id: String::new(),
            style_id: style.into(),
            enabled: true,
            value: 0.0,
            min: 0.0,
            max: 1.0,
            surface: None,
        }
    }
    pub fn card(id: &str, rect: [f32; 4], style: &str) -> Self {
        Self::new(id, Kind::Card, rect, style)
    }
    pub fn text(id: &str, rect: [f32; 4], text: &str, style: &str) -> Self {
        let mut w = Self::new(id, Kind::Text, rect, style);
        w.text = text.into();
        w
    }
    pub fn icon(id: &str, rect: [f32; 4], icon_id: &str, style: &str) -> Self {
        let mut w = Self::new(id, Kind::Icon, rect, style);
        w.icon_id = icon_id.into();
        w
    }
    pub fn button(id: &str, rect: [f32; 4], text: &str, action: &str, style: &str) -> Self {
        let mut w = Self::new(id, Kind::Button, rect, style);
        w.text = text.into();
        w.action_id = action.into();
        w
    }
    pub fn slider(
        id: &str,
        rect: [f32; 4],
        value: f32,
        min: f32,
        max: f32,
        action: &str,
        style: &str,
    ) -> Self {
        let mut w = Self::new(id, Kind::Slider, rect, style);
        w.value = value.clamp(min, max);
        w.min = min;
        w.max = max;
        w.action_id = action.into();
        w
    }
    pub fn surface(id: &str, rect: [f32; 4], generation: &str, surface_id: &str) -> Self {
        let mut w = Self::new(id, Kind::SurfaceReference, rect, "panel.card");
        w.surface = Some((generation.into(), surface_id.into()));
        w
    }
    pub fn parent(mut self, id: &str) -> Self {
        self.parent_id = id.into();
        self
    }
    pub fn enabled(mut self, value: bool) -> Self {
        self.enabled = value;
        self
    }
    pub fn with_icon(mut self, id: &str) -> Self {
        self.icon_id = id.into();
        self
    }
    pub fn upsert(&self) -> Value {
        let mut node = json!({"op":"upsert_node", "node_id":self.id, "parent_id":self.parent_id,
            "kind":self.kind, "rect":self.rect, "style_id":self.style_id, "enabled":self.enabled});
        if !self.text.is_empty() {
            node["text"] = self.text.clone().into();
        }
        if !self.icon_id.is_empty() {
            node["icon_id"] = self.icon_id.clone().into();
        }
        if !self.action_id.is_empty() {
            node["action_id"] = self.action_id.clone().into();
        }
        if self.kind == Kind::Slider {
            node["value"] = self.value.into();
            node["min"] = self.min.into();
            node["max"] = self.max.into();
        }
        if let Some((generation, surface)) = &self.surface {
            node["window_server_generation"] = generation.clone().into();
            node["surface_id"] = surface.clone().into();
        }
        node
    }
}

#[derive(Clone, Debug)]
pub struct Scene {
    pub logical_size: [f32; 2],
    pub size_m: [f32; 2],
    pub position_m: [f32; 3],
    pub policy: String,
    pub widgets: Vec<Widget>,
}
impl Scene {
    pub fn new(
        logical_size: [f32; 2],
        size_m: [f32; 2],
        position_m: [f32; 3],
        policy: &str,
    ) -> Self {
        Self {
            logical_size,
            size_m,
            position_m,
            policy: policy.into(),
            widgets: Vec::new(),
        }
    }
    pub fn push(&mut self, widget: Widget) {
        self.widgets.push(widget);
    }
    pub fn operations(&self) -> Vec<Value> {
        let mut result = vec![json!({"op":"set_root_policy", "policy":self.policy,
            "logical_size":self.logical_size, "size_m":self.size_m, "position_m":self.position_m})];
        result.extend(self.widgets.iter().map(Widget::upsert));
        result
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    Hover,
    Press,
    Drag,
    Release,
    Cancel,
}
#[derive(Clone, Debug)]
pub struct PointerEvent {
    pub phase: Phase,
    pub interaction_id: String,
    pub local: [f32; 2],
}
#[derive(Clone, Debug, Default)]
pub struct InteractionState {
    pub hovered: bool,
    pub focused: bool,
    pub pressed: bool,
    pub interaction_id: Option<String>,
}
#[derive(Clone, Debug, PartialEq)]
pub enum Action {
    Activated(String),
    ValueChanged { action_id: String, value: f32 },
    Cancelled,
}
impl InteractionState {
    pub fn apply(&mut self, widget: &mut Widget, event: &PointerEvent) -> Option<Action> {
        if event.phase == Phase::Cancel || !widget.enabled {
            self.hovered = false;
            self.pressed = false;
            self.interaction_id = None;
            return Some(Action::Cancelled);
        }
        let inside = event.local[0].is_finite()
            && event.local[1].is_finite()
            && event.local[0] >= 0.0
            && event.local[1] >= 0.0
            && event.local[0] <= widget.rect[2]
            && event.local[1] <= widget.rect[3];
        match event.phase {
            Phase::Hover => {
                self.hovered = inside;
                None
            }
            Phase::Press if inside && !self.pressed => {
                self.pressed = true;
                self.interaction_id = Some(event.interaction_id.clone());
                self.slider_value(widget, event)
            }
            Phase::Drag
                if self.pressed
                    && self.interaction_id.as_deref() == Some(&event.interaction_id) =>
            {
                self.slider_value(widget, event)
            }
            Phase::Release
                if self.pressed
                    && self.interaction_id.as_deref() == Some(&event.interaction_id) =>
            {
                self.pressed = false;
                self.interaction_id = None;
                if widget.kind == Kind::Slider {
                    self.slider_value(widget, event)
                } else if widget.kind == Kind::Button && inside {
                    Some(Action::Activated(widget.action_id.clone()))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
    fn slider_value(&self, widget: &mut Widget, event: &PointerEvent) -> Option<Action> {
        if widget.kind != Kind::Slider || !event.local[0].is_finite() || widget.rect[2] <= 0.0 {
            return None;
        }
        let value = widget.min
            + (widget.max - widget.min) * (event.local[0] / widget.rect[2]).clamp(0.0, 1.0);
        if (value - widget.value).abs() <= f32::EPSILON {
            return None;
        }
        widget.value = value;
        Some(Action::ValueChanged {
            action_id: widget.action_id.clone(),
            value,
        })
    }
    pub fn cancel(&mut self) {
        self.hovered = false;
        self.pressed = false;
        self.interaction_id = None;
    }
}
