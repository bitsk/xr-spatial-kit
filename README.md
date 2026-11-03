# xr-spatial-kit

Rust 库与 UKUI 空间样式数据，不创建 OpenXR、GPU、socket 或常驻进程。

`Widget` 提供 Card、Text、Icon、Button、Slider、SurfaceReference；rect 使用组件平面的逻辑像素，+X 向右、+Y 向下。`Scene` 提供根策略和原子事务的操作列表，交给 `xr-shell-protocols` 客户端发送；协议、版本协商和渲染由 Shell 维护。

```rust
use xr_spatial_kit::{Scene, Widget};
let mut scene = Scene::new([1200.0, 180.0], [1.2, 0.18], [0.0, 1.1, -1.5], "dock_lazy_follow");
scene.push(Widget::button("launcher", [16.0, 16.0, 100.0, 80.0], "应用", "open-launcher", "dock.application"));
let operations = scene.operations();
```

`InteractionState` 将同一 interaction 的 press/drag/release 和 Cancel 归一；按钮在内部释放时激活，滑块捕获后允许移出区域继续调整。取消会清空按下状态。客户端需要将 Shell 事件的 node-local 坐标和 capture/interaction 标识交给它。

`Theme::default()` 使用内建 UKUI tokens；`Theme::load()` 可加载独立安装的样式文件。字体族含中文回退名称，实际字形塑形与绘制由 Shell 的 Pango/Cairo 后端完成；单独安装 token 文件不能证明中文已经呈现。

```sh
cargo build --release --offline --locked
dpkg-buildpackage -us -uc -b
```

构建前准备固定 Cargo.lock 的缓存。`librust-xr-spatial-kit-dev` 安装源码与样式输入，`xr-spatial-style-ukui` 安装 `/usr/share/xr/styles/ukui.json`。包构建、与真实 Shell 的控件行为、目标系统安装分别记录；包安装不会启动桌面。
