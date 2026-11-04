// SPDX-License-Identifier: GPL-3.0-or-later
//! Three independent capsules, aligned in STAGE space with open gaps between them.
pub const LOGICAL_HEIGHT: f32 = 72.0;
pub const CENTER_LOGICAL: [f32; 2] = [620.0, LOGICAL_HEIGHT];
pub const SIDE_LOGICAL: [f32; 2] = [440.0, LOGICAL_HEIGHT];
pub const SCALE_M: f32 = 0.0016;
pub const CENTER_SIZE: [f32; 2] = [0.992, 0.1152];
pub const SIDE_SIZE: [f32; 2] = [0.704, 0.1152];
pub const CENTER_POSITION: [f32; 3] = [0.0, 0.86, -1.72];
pub const LEFT_POSITION: [f32; 3] = [-0.9632, 0.86, -1.72];
pub const RIGHT_POSITION: [f32; 3] = [0.9632, 0.86, -1.72];
pub const ROTATION: [f32; 3] = [12.0, 0.0, 0.0];
