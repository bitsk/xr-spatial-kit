# XR spatial components

Maintain the Rust control model, local layout and UKUI style data here. This
library does not own an OpenXR session, GPU, OS service or component socket.
Shell owns the wire protocol and rendering. Preserve real Git identity/dates,
source licenses and independent offline packaging. Public files contain no
private host paths, addresses or credentials. Record builds separately from
real scene, input and target-system acceptance.

## XR Spatial Engineering Constraints

Follow `openkylin-xr-docs/wiki/xr-spatial-guidelines-and-issues.md`:
- **Angular Sizing (`dmm`) & Hit Targets**: Define typography and control dimensions with angular consistency (`1 dmm = 1 mm at 1 m`); far-field ray targets must provide at least `2.2°` angular hit size (with hit-slop padding for compact icons).
- **Spherical Placement & 3D Depth States**: Support view-relative spawn policies (`head_horizon_spawn`), user-facing orientation (`user_facing_billboard`), spherical offsets (`yaw_deg, pitch_deg, distance_m`), and distinct 3D Z-travel states for default, hover-lift, and press-inward travel.
