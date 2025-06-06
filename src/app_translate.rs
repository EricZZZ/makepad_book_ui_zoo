use makepad_widgets::*; // Import Makepad Widgets package

// Define live_design macro for declaring UI components and layout
live_design! {
// import Makepad theme and shaders, and widgets
use link::theme::*;
use link::shaders::*;
use link::widgets::*;

// 定义 App 组件
    App = {{App}} {
        // 定义 UI 树的根节点
        ui: <Root>{
            // 定义主窗口
            main_window = <Window>{
                // 显示背景
                show_bg: true
                width: Fill,
                height: Fill


                // 定义自定义背景绘制
                draw_bg: {
                    fn pixel(self) -> vec4 {
                        // 获取几何位置
                        let st = vec2(
                            self.geom_pos.x,
                            self.geom_pos.y
                        );

                        // 计算颜色，基于 x 和 y 位置及时间
                        let color = vec3(st.x, st.y, abs(sin(self.time)));
                        return vec4(color, 1.0);
                    }
                                }
                // 定义窗口主体
                body = <ScrollXYView>{
                    // 布局方向为垂直
                    flow: Down,
                    // 子项间距为10
                    spacing:10,
                    // 对齐方式
                    align: {
                        x: 0.5,
                        y: 0.5
                    },
                    // 按钮组件
                    button1 = <Button> {
                        text: "Hello world"
                        draw_text:{color:#f00} // 文字颜色为红色
                    }


                    // 文本输入组件
                    label1 = <Label> {
                        draw_text: {
                            color: #f // 文字颜色为白色
                        },
                        text: "Click to count "
                    }

                    // 文本输入组件
                    input1 = <TextInput> {
                        width: 100, height: 30
                        text: "Counter: 0 "
                    }
                }
            }
        }
    }
}

// Define App struct containing UI and counter
#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,

    #[rust]
    counter: usize, // 计数器
}

// 实现 LiveRegister trait，用于注册 live desin
impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        // Register Makepad Widgets' live design
        makepad_widgets::live_design(cx);
    }
}

// 实现 AppMain 特性，用于处理事件
impl AppMain for App {
    // 处理 Makepad 事件
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        // 匹配事件并处理
        self.match_event(cx, event);
        // 处理 UI 事件
        self.ui.handle_event(cx, event, &mut Scope::empty());

        match event {
            Event::KeyDown(key_event) => {
                println!("Key pressed: {:?}", key_event.key_code);
            }
            Event::KeyUp(key_event) => {
                println!("Key released: {:?}", key_event.key_code);
            }
            _ => {}
        }
    }
}

// 实现 MatchEvent 特性，用于处理事件
impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        // 检查按钮是否被点击
        // 这里可以直接通过 `id!()`使用 button1 实例，获取被点击 Button
        // `clicked` 是 Button 组件的方法
        if self.ui.button(id!(button1)).clicked(actions) {
            // 增加计数器
            log!("BUTTON jk {}", self.counter);
            self.counter += 1;
            // 更新标签文本
            // 同样，通过 `id!()` 获取 input1 文本输入实例
            let input = self.ui.text_input(id!(input1));
            // 通过文本输入框组件内置的 `set_text`
            // 更新文本输入框的内容为新的计数器值。
            input.set_text(cx, format!("Counter: {}", self.counter).as_str());
        }
    }

    // 程序启动时调用
    fn handle_startup(&mut self, _cx: &mut Cx) {
        println!("App started");
    }
}
// Define application entry point
app_main!(App);
