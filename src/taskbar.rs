// SPDX-License-Identifier: GPL-3.0-or-later
//! Three capsules share one Shell-owned lazy-follow anchor and retain arc-length gaps.
pub const POLICY: &str = "dock_lazy_follow";
pub const LOGICAL_HEIGHT: f32 = 72.0;
pub const CENTER_LOGICAL: [f32; 2] = [620.0, LOGICAL_HEIGHT];
pub const SIDE_LOGICAL: [f32; 2] = [340.0, LOGICAL_HEIGHT];
pub const SCALE_M: f32 = 0.0016;
pub const CENTER_SIZE: [f32; 2] = [0.992, 0.1152];
pub const SIDE_SIZE: [f32; 2] = [0.544, 0.1152];
pub const CENTER_POSITION: [f32; 3] = [0.0, 0.86, -1.72];
pub const LEFT_POSITION: [f32; 3] = [-0.77718075, 0.86, -1.53440219];
pub const RIGHT_POSITION: [f32; 3] = [0.77718075, 0.86, -1.53440219];
pub const ROTATION: [f32; 3] = [12.0, 0.0, 0.0];

/// Horizontal track radius; the eye-to-track distance also includes height.
pub const RADIUS_M: f32 = 1.72;
pub const GAP_PX: f32 = 24.;
