// SPDX-License-Identifier: GPL-3.0-or-later
//! Three capsules share one Shell-owned lazy-follow anchor and retain arc-length gaps.
pub const POLICY: &str = "dock_lazy_follow";
pub const LOGICAL_HEIGHT: f32 = 72.0;
pub const CENTER_LOGICAL: [f32; 2] = [620.0, LOGICAL_HEIGHT];
pub const SIDE_LOGICAL: [f32; 2] = [340.0, LOGICAL_HEIGHT];
pub const SCALE_M: f32 = 0.0016;
pub const CENTER_SIZE: [f32; 2] = [0.992, 0.1152];
pub const SIDE_SIZE: [f32; 2] = [0.544, 0.1152];
/// Horizontal track radius; the eye-to-track distance also includes height.
pub const RADIUS_M: f32 = 2.2;
pub const GAP_PX: f32 = 24.;
const EYE_DROP_M: f32 = 0.62;
const CENTER_HEIGHT_M: f32 = 1.6 - EYE_DROP_M;
pub const EYE_TO_CENTER: [f32; 3] = [0.0, -EYE_DROP_M, -RADIUS_M];
pub const CENTER_POSITION: [f32; 3] = [0.0, CENTER_HEIGHT_M, -RADIUS_M];
// Side centres lie at ±(half-centre width + gap + half-side width) along the arc.
pub const RIGHT_POSITION: [f32; 3] = [0.78846353, CENTER_HEIGHT_M, -2.0538561];
pub const LEFT_POSITION: [f32; 3] = [-RIGHT_POSITION[0], CENTER_HEIGHT_M, RIGHT_POSITION[2]];
// Positive pitch leaves the upper edge farther away and aims the face at eye height.
pub const ROTATION: [f32; 3] = [15.738801, 0.0, 0.0];
