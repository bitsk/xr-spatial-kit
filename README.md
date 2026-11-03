# xr-spatial-kit

晴空控件的视觉基准见 `design/qingkong-controls/control-library.png`。
该目录保留用户风格板、内置绘图提示词、分层材质和应用图标候选，以及带区域坐标的色彩对照。
浅色雾银玻璃与墨蓝正文是这一主题的基准；图稿中的时间、天气、电量均为设计示例。
图稿完成、纹理切分质量与原生渲染验收分别记录，不以参考图替代真实画面。

Rust 库与 UKUI 空间样式数据，不创建 OpenXR、GPU、socket 或常驻进程。

`Widget` 提供 Group、Card、Text、Icon、Button、Slider、SurfaceReference；rect 使用组件平面的逻辑像素，+X 向右、+Y 向下。`Scene` 提供根策略和原子事务的操作列表，交给 `xr-shell-protocols` 客户端发送；协议、版本协商和渲染由 Shell 维护。

```rust
use xr_spatial_kit::{Scene, Widget, taskbar};
let mut scene = Scene::new(taskbar::CENTER_LOGICAL, taskbar::CENTER_SIZE, taskbar::CENTER_POSITION, "dock_lazy_follow")
    .with_rotation(taskbar::ROTATION);
scene.push(Widget::spatial_group("apps", [960.0, 584.0], [0.96, 0.584], [0.0, 1.25, -1.68]));
scene.push(Widget::button("files", [24.0, 24.0, 180.0, 64.0], "文件管理器", "launch-files", "button.secondary")
    .with_icon("system-file-manager").parent("apps"));
let operations = scene.operations();
```

`Widget::spatial_group(id, logical_size, size_m, position_m)` 开启独立世界面板，其子控件使用 `.parent(id)` 关联和局部像素坐标。`Scene` 与 Group 的 `.with_rotation([pitch, yaw, roll])` 使用角度，默认零；正 pitch 表示面板朝上。Group 本身只定义坐标，背景用 Card 单独绘制；关闭时删除该子树。

`InteractionState` 将同一 interaction 的 press/drag/release 和 Cancel 归一；按钮在内部释放时激活，滑块捕获后允许移出区域继续调整。取消会清空按下状态。客户端需要将 Shell 事件的 node-local 坐标和 capture/interaction 标识交给它。

晴空空间样式使用雾银 `#E8F0F8` 半透明玻璃、暖白 `#F5F7FA` 键帽、墨蓝 `#233047` 正文与焦点蓝 `#3790FA`。宿主 GTK 外观切换不覆盖这一空间材质。`Theme::radius(role)` 提供 panel=20、button=12、key=12、dock=36 的圆角 tokens；72px 三岛背景按半高完整胶囊渲染。`taskbar` 模块共享三个岛的独立坐标和物理尺寸，岛间保留52逻辑像素的间隙。交互浮起由 Shell 几何后端完成。

`Theme::default()` 使用内建 UKUI tokens；`Theme::load()` 可加载独立安装的样式文件。字体族含中文回退名称，实际字形塑形与绘制由 Shell 的 Pango/Cairo 后端完成；单独安装 token 文件不能证明中文已经呈现。

```sh
cargo build --release --offline --locked
dpkg-buildpackage -us -uc -b
```

构建前准备固定 Cargo.lock 的缓存。`librust-xr-spatial-kit-dev` 安装源码与样式输入，`xr-spatial-style-ukui` 安装 `/usr/share/xr/styles/ukui.json`，以及 `/usr/share/xr/art/qingkong/` 下的原始绘图 PNG 和切片区域清单。Shell 按清单裁切并建立独立透明采样边距，文字与动态状态不包含在底图中。包构建、与真实 Shell 的控件行为、目标系统安装分别记录；包安装不会启动桌面。
