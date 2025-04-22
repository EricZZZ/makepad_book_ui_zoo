use makepad_widgets::*;

live_design! {
    // import Makepad theme and shaders, 以及 widgets
    use link::theme::*;
    use link::shaders::*;
    use link::widgets::*;

    // define color of background container
    COLOR_CONTAINER = (THEME_COLOR_D_1)

    DEMO_COLOR_1 = #8f0
    DEMO_COLOR_2 = #0f8
    DEMO_COLOR_3 = #80f

    ZooGroup = <RoundedView> {
        height: Fit,width: Fill,
        flow: Right,
        align: {x: 0.0, y: 0.5},
        margin: 0.,
        show_bg: false;
        draw_bg: {
            color: (COLOR_CONTAINER)
        }
    }

    ZooBlock = <RoundedView> {
        width: 50.,height: 50.
        margin: 0.,
        spacing: 0.,

        show_bg: true;
        draw_bg: {
            fn get_color(self) -> vec4 {
                return mix(self.color, self.color*0.5, self.pos.y);
            }
            radius: (THEME_CONTAINER_CORNER_RADIUS)
        }
    }

    ZooTitle = <View> {
        width: Fill,
        height: Fit,
        margin: 10.0,
        title = <H2> {
            text: "Makepad UI Zoo"
        }
    }

    ZooHeader = <View> {
        width: Fill,
        height: Fit,
        flow: Down,
        spacing: 10.,
        margin: {
            top: 9,
            bottom: 9,
            left: 9,
            right: 9
        }
        divider = <Hr>{ }
        title = <H3> { }
    },

    ZooDesc = <P> { },

    App = {{App}} {
        ui: <Window> {
            show_bg: true,
            draw_bg: {
                // color: #970707    // dark red
                // color: vec3(0.592, 0.027, 0.027) // dark red as above
                // color: vec3(1, 1, 0) // yellow
                // color: #000 // black
                }
            window: {
                title: "Makepad UI zoo"
            },
            caption_bar = {
                visible: true,
                margin: {left: -500},
                caption_label = { label = {text: "Makepad book UI Zoo caption bar"} },
            },

            body = <View> {
                width: Fill, height: Fill,
                flow: Down, //child components will be arranged vertically
                spacing: 10.,   //spacing between child components
                margin: 0.,
                scroll_bars: <ScrollBars> {}     //margin around the component

                <ZooTitle> {}

                <ZooHeader> {
                    title = {text: "Intro"}
                    <ZooDesc> {
                        text: "Intro."
                    }
                    <View> {
                        width: Fill, height: Fit,
                        flow: Down,
                        <P> { text: "- Shader-based: what does that mean for how things work." }
                        <P> { text: "- Inheritance mechanisms in the DSL." }
                        <P> { text: "- Introduction to the layout system." }
                        <P> { text: "- Base theme parameters." }
                        <P> { text: "- Typographic system. Base font-size and contrast." }
                        <P> { text: "- Space constants to control denseness of the design." }
                        <P> { text: "- Transparency mechanism of the widgets. Nesting for structure." }
                    }
                }

                <ZooHeader> {
                    title = {text: "Control Heights & Text Baselines"}
                    <ZooDesc> {
                        text: "Control heights and text baselines"
                    }
                    <View> {
                        width: Fill, height: Fit,
                        align: { x: 0., y: 0.}
                        flow: Right,
                        spacing: (THEME_SPACE_2)
                        <P> { text: "TestLabel", width: Fit}
                        <Vr> {}
                        <LinkLabel> { text: "TestButton", width: Fit}
                        <CheckBox> { text: "TestButton"}
                        <CheckBoxToggle> { text: "TestButton"}
                        <ButtonFlat> { text: "TestButton"}
                        <Button> { text: "TestButton"}
                        <TextInput> { text: "TestButton"}
                        <Slider> { text: "TestButton"}
                        <SliderBig> { text: "TestButton"}
                    }
                }

                <ZooHeader> {
                    title = {text: "Typography"}
                    <ZooDesc> {
                        text: "Typography."
                    }
                    <View> {
                        width: Fill, height: Fit,
                        flow: Down,
                        <H1> { text: "H1 headline" }
                        <H1italic> { text: "H1 italic headline" }
                        <H2> { text: "H2 headline" }
                        <H2italic> { text: "H2 italic headline" }
                        <H3> { text: "H3 headline" }
                        <H3italic> { text: "H3 italic headline" }
                        <H4> { text: "H4 headline" }
                        <H4italic> { text: "H4 italic headline" }
                        <P> { text: "P copy text" }
                        <Pitalic> { text: "P italic copy text" }
                        <Pbold> { text: "P bold copy text" }
                        <Pbolditalic> { text: "P bold italic copy text" }
                    }
                }

                <ZooHeader> {
                    title = {text: "<View>" }
                    <ZooDesc> {text:"This is a gray view with flow set to Right\nTo show the extend, the background has been enabled using show_bg and a gray pixelshader has been provided to draw_bg."}
                    <View> {
                        height: Fit
                        flow: Right,
                        show_bg: true,
                        draw_bg: { color: (COLOR_CONTAINER) }
                        padding: 10.
                        spacing: 10.
                        <ZooBlock> {draw_bg:{color: (DEMO_COLOR_1)}}
                        <ZooBlock> {draw_bg:{color: (DEMO_COLOR_2)}}
                        <ZooBlock> {draw_bg:{color: (DEMO_COLOR_3)}}
                    }

                    <ZooDesc> {text:"This utlizes a <Filler> to separate items."}
                    <View> {
                        height: Fit
                        flow: Right,
                        show_bg: true,
                        draw_bg: {
                            color: (COLOR_CONTAINER)
                        }
                        padding: 10.
                        spacing: 10.
                        <ZooBlock> {draw_bg:{color: (DEMO_COLOR_1)}}
                        <Filler> {} // placeholder
                        <ZooBlock> {draw_bg:{color: (DEMO_COLOR_2)}}
                        <ZooBlock> {draw_bg:{color: (DEMO_COLOR_3)}}
                    }

                    <ZooDesc> {text:"This view is bigger on the inside"}
                    <View> {
                        width: 150, height: 150,
                        flow: Right,
                        padding: 10.,
                        spacing: 10.,

                        show_bg: true,
                        draw_bg: {
                            color: (COLOR_CONTAINER)
                        }
                        scroll_bars: <ScrollBars> {}

                        <View> {
                            width: Fit, height: Fit,
                            flow: Down,
                            show_bg: false,
                            spacing: 10
                            <ZooBlock> {draw_bg:{color: (DEMO_COLOR_1)}}
                            <ZooBlock> {draw_bg:{color: (DEMO_COLOR_2)}}
                            <ZooBlock> {draw_bg:{color: (DEMO_COLOR_3)}}
                            <ZooBlock> {draw_bg:{color: (DEMO_COLOR_2)}}
                        }

                        <View> {
                            width: Fit, height: Fit,
                            flow: Down,
                            show_bg: false,
                            spacing: 10
                            <ZooBlock> {draw_bg:{color: (DEMO_COLOR_1)}}
                            <ZooBlock> {draw_bg:{color: (DEMO_COLOR_2)}}
                            <ZooBlock> {draw_bg:{color: (DEMO_COLOR_3)}}
                            <ZooBlock> {draw_bg:{color: (DEMO_COLOR_2)}}
                        }

                        <View> {
                            width: Fit, height: Fit,
                            flow: Down,
                            show_bg: false,
                            <ZooBlock> {draw_bg:{color: (DEMO_COLOR_1)}}
                            <ZooBlock> {draw_bg:{color: (DEMO_COLOR_2)}}
                            <ZooBlock> {draw_bg:{color: (DEMO_COLOR_3)}}
                            <ZooBlock> {draw_bg:{color: (DEMO_COLOR_2)}}
                        }

                        <View> {
                            width: Fit, height: Fit,
                            flow: Down,
                            show_bg: false,
                            spacing: 10
                            <ZooBlock> {draw_bg:{color: (DEMO_COLOR_1)}}
                            <ZooBlock> {draw_bg:{color: (DEMO_COLOR_2)}}
                            <ZooBlock> {draw_bg:{color: (DEMO_COLOR_3)}}
                            <ZooBlock> {draw_bg:{color: (DEMO_COLOR_2)}}
                        }

                        <View> {
                            width: Fit, height: Fit,
                            flow: Down,
                            show_bg: false,
                            spacing: 10
                            <ZooBlock> {draw_bg:{color: (DEMO_COLOR_1)}}
                            <ZooBlock> {draw_bg:{color: (DEMO_COLOR_2)}}
                            <ZooBlock> {draw_bg:{color: (DEMO_COLOR_3)}}
                        }
                    }
                }

                <ZooHeader> {
                    title = {text: "<TextInput> with interaction"}
                    padding: 10.
                    <View> {
                        height: Fit, width: Fill,
                        spacing: (THEME_SPACE_2),
                        textalreadyfilled = <TextInput> {
                            text: "text here"
                        }
                        simpletextinput = <TextInput> {
                            width: Fill,
                            empty_message: "Enter text here"
                        }
                        simpletextinput_outputbox = <P> {
                            text: "Output"
                        }
                    }
                }

                <ZooHeader> {
                    title = {text:"<Button>"}
                    <ZooDesc> {text:"A small clickable region"}
                    <ZooGroup> {
                        flow: Down,
                        width: Fill, height: Fit,
                        align: { x: 0.0, y: 0.5},
                        spacing: 10.,

                        <H4> { text: "Default" }
                        <Label> { text: "<Button>" }
                        basicbutton = <Button> { text: "I can be clicked" }

                        <H4> { text: "Button with an icon"}
                        <Label> { text: "<ButtonIcon>"}
                        iconbutton = <ButtonIcon> {
                            draw_icon: {
                                color: #ff0,
                                svg_file: dep("crate://self/resources/Icon_Favorite.svg"),
                            }
                            text: "I can have a icon!"
                        }

                        <H4> { text: "Flat Mode"}
                        <Label> { text: "<ButtonFlat>"}
                        <View> {
                            flow: Right,
                            align: { x: 0.0, y: 0.5},
                            width: Fill, height: Fit,
                            <ButtonFlat> {
                                draw_icon: {
                                    color: #f00,
                                    svg_file: dep("crate://self/resources/Icon_Favorite.svg"),
                                }
                                text: "I can have a lovely icon!"
                            }

                            <ButtonFlat> {
                                draw_icon: {
                                    svg_file: dep("crate://self/resources/Icon_Favorite.svg"),
                                }
                            }

                            <ButtonFlat> {
                                flow: Down,
                                icon_walk: { width: 15. }
                                draw_icon: {
                                    svg_file: dep("crate://self/resources/Icon_Favorite.svg"),
                                }
                                text: "Vertical Layout"
                            }
                        }

                        <H4> { text: "Freely styled button"}
                        <Label> { text: "<Button>"}
                        styledbutton = <Button> {
                            draw_bg: {
                                fn pixel(self) -> vec4 {
                                    return (THEME_COLOR_MAKEPAD) + self.pressed * vec4(1., 1., 1., 1.)
                                }
                            }
                            draw_text: {
                                fn get_color(self) -> vec4 {
                                    return (THEME_COLOR_U_5) - vec4(0., 0.1, 0.4, 0.) * self.hover - vec4(1., 1., 1., 0.);
                                }
                            }
                            text: "I can be styled!"
                        }


                    }
                }

                <ZooHeader> {
                    title = {text: "<Label>"}
                    <ZooDesc> {text:"Default single line textbox"}
                    <ZooGroup> {
                        <Label> { text: "This is a small line of text" }
                    }
                    <ZooGroup> {
                        <Label> {
                            draw_text: {
                                color: #fff,
                                text_style: {
                                    font: {path: dep("crate://self/resources/XITSOneText-BoldItalic.ttf")},
                                    font_size: 20,
                                }
                            }
                            text: "You can style text using colors and fonts"
                        }
                    }
                    <ZooGroup> {
                        <Label> {
                            draw_text: {
                                fn get_color(self) ->vec4{
                                    return mix((THEME_COLOR_MAKEPAD), (THEME_COLOR_U_HIDDEN), self.pos.x)
                                }
                                color: (THEME_COLOR_MAKEPAD)
                                text_style: {
                                    font_size: 40.,
                                }
                            }
                            text: "OR EVEN SOME PIXELSHADERS"
                        }
                    }
                }

                <ZooHeader> {
                    title = { text:"<Slider>" }
                    <ZooDesc> { text:"A parameter dragger" }
                    <ZooGroup> {
                        width: Fill, height: Fit,
                        flow: Right,
                        spacing: 10.0,
                        align: { x: 0., y: 0.}
                        // <View> {
                        //     width: Fill, height: Fit,
                        //     flow: Down,
                        //     <Slider> { text: "Default" }
                        //     <Slider> { text: "min 0 to max 100", min: 0., max: 100. }
                        //     <Slider> { text: "precision 7", precision: 4 }  // max precision
                        //     <Slider> { text: "stepped 0.1", step: 0.1 }
                        // }
                        <View> {
                            width: Fill, height: Fit,
                            flow: Down,
                            <SliderBig> { text: "Default 0.2", default: 0.2 }
                            <SliderBig> { text: "min -50 to max 50", min: -50, max: 50. }
                            <SliderBig> { text: "precision 8", precision: 8 }   // when greater than 7, auto jump to 16 and won't changed
                            <SliderBig> { text: "stepped 0.2", step: 0.2 }
                        }
                        <View> {
                            width: Fill, height: Fit,
                            flow: Down,
                            <SliderAlt1> {
                                text: "Colored",
                                draw_slider: {
                                    val_color_a: (#FFCC00),
                                    val_color_b: #f00,
                            }
                            }
                        }
                    }

                }

                <ZooHeader> {
                    title = {text:"<DropDown>"}
                    <ZooDesc> {text:"DropDown control. This control currently needs to be databound which needs some plumbing. In this sample there is a binding context struct in the main app struct - which gets bound on app start - and updated during handle_actions."}
                    <ZooGroup> {
                        dropdown = <DropDown> {
                            labels: ["Value One", "Value Two", "Thrice", "Fourth Value", "Option E", "Hexagons"],
                            values: [ValueOne, ValueTwo, Thrice, FourthValue, OptionE, Hexagons]
                        }
                    }
                }
                <ZooHeader> {
                    title = {text:"Place holder"}
                    <ZooDesc> {text:"Place holder for drop down"}

                }

                <ZooHeader> {
                    title = {text:"<Html>"}
                    <ZooDesc> {text:"HTML Widget"}
                    <ZooGroup> {
                        <Html> {
                            width:Fill, height:Fit,
                            body:"<H1>H1 Headline</H1><H2>H2 Headline</H2><H3>H3 Headline</H3><H4>H4 Headline</H4><H5>H5 Headline</H5><H6>H6 Headline</H6>This is <b>bold</b>&nbsp;and <i>italic text</i>.<sep><b><i>Bold italic</i></b>, <u>underlined</u>, and <s>strike through</s> text. <p>This is a paragraph</p> <code>A code block</code>. <br/> And this is a <a href='https://www.google.com/'>link</a><br/><ul><li>lorem</li><li>ipsum</li><li>dolor</li></ul><ol><li>lorem</li><li>ipsum</li><li>dolor</li></ol><br/> <blockquote>Blockquote</blockquote> <pre>pre</pre><sub>sub</sub><del>del</del>"
                        }
                    }
                }

                <ZooHeader> {
                    title = {text:"<Markdown>"}
                    <ZooDesc> {text:"Markdown"}
                    <ZooGroup> {
                        <Markdown> {
                            width:Fill, height: Fit,
                            body:"# Headline 1 \n ## Headline 2 \n ### Headline 3 \n #### Headline 4 \n This is standard text with a  \n\n line break a short ~~strike through~~ demo.\n\n *Italic text* \n\n **Bold text** \n\n - Bullet\n - Another bullet\n\n - Third bullet\n\n 1. Numbered list Bullet\n 2. Another list entry\n\n 3. Third list entry\n\n `Monospaced text`\n\n> This is a quote.\n\nThis is `inline code`.\n\n ```code block
                            ```"
                        }
                    }
                }

                <ZooHeader> {
                    title = {text:"<Image>"}
                    <ZooDesc> {text:"A static inline image from a resource."}
                    <ZooGroup> {
                        height: Fit, width: Fill,
                        spacing: (THEME_SPACE_2)
                        scroll_bars: <ScrollBars> {}
                        <View> {
                            width: Fit, height: Fit, flow: Down,
                            <View> {
                                show_bg: true, draw_bg: { color: (THEME_COLOR_BG_CONTAINER)}, width: 125, height: 250, flow: Down,
                                <Image> { source: dep("crate://self/resources/ducky.png" ) }
                            }
                            <P> { text: "Default" }
                        }
                        <View> {
                            width: Fit, height: Fit, flow: Down,
                            <View> {
                                show_bg: true, draw_bg: { color: (THEME_COLOR_BG_CONTAINER)}, width: 125, height: 250,
                                <Image> { height: Fill, source: dep("crate://self/resources/ducky.png" ), min_height: 100 }
                            }
                            <P> { text: "min_height: 100" } // TODO: get this to work correctly
                        }
                        <View> {
                            width: Fit, height: Fit, flow: Down,
                            <View> {
                                show_bg: true, draw_bg: { color: (THEME_COLOR_BG_CONTAINER)}, width: 125, height: 250,
                                <Image> { width: Fill, source: dep("crate://self/resources/ducky.png" ), width_scale: 1.5 }
                            }
                            <P> { text: "width_scale: 1.5" } // TODO: get this to work correctly
                        }
                        <View> {
                            width: Fit, height: Fit, flow: Down,
                            <View> {
                                show_bg: true, draw_bg: { color: (THEME_COLOR_BG_CONTAINER)}, width: 125, height: 250,
                                <Image> { width: Fill, height: Fill, source: dep("crate://self/resources/ducky.png"), fit: Stretch }
                            }
                            <P> { text: "fit: Stretch" }
                        }
                        <View> {
                            width: Fit, height: Fit, flow: Down,
                            <View> {
                                show_bg: true, draw_bg: { color: (THEME_COLOR_BG_CONTAINER)}, width: 125, height: 250,
                                <Image> { width: Fill, height: Fill, source: dep("crate://self/resources/ducky.png" ), fit: Horizontal }
                            }
                            <P> { text: "fit: Horizontal" }
                        }
                        <View> {
                            width: Fit, height: Fit, flow: Down,
                            <View> {
                                show_bg: true, draw_bg: { color: (THEME_COLOR_BG_CONTAINER)}, width: 125, height: 250,
                                <Image> { width: Fill, height: Fill, source: dep("crate://self/resources/ducky.png" ), fit: Vertical }
                            }
                            <P> { text: "fit: Vertical" }
                        }
                        <View> {
                            width: Fit, height: Fit, flow: Down,
                            <View> {
                                show_bg: true, draw_bg: { color: (THEME_COLOR_BG_CONTAINER)}, width: 125, height: 250,
                                <Image> { width: Fill, height: Fill, source: dep("crate://self/resources/ducky.png" ), fit: Smallest }
                            }
                            <P> { text: "fit: Smallest" }
                        }
                        <View> {
                            width: Fit, height: Fit, flow: Down,
                            <View> {
                                show_bg: true, draw_bg: { color: (THEME_COLOR_BG_CONTAINER)}, width: 125, height: 250,
                                <Image> { width: Fill, height: Fill, source: dep("crate://self/resources/ducky.png" ), fit: Biggest }
                            }
                            <P> { text: "fit: Biggest" }
                        }
                    }
                }
            }

        }
    }
}

#[derive(Live, LiveHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
    #[rust]
    counter: usize,
}

impl LiveRegister for App {
    fn live_register(cx: &mut Cx) {
        // Register Makepad Widgets' live design
        makepad_widgets::live_design(cx);
    }
}

impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if let Some(txt) = self.ui.text_input(id!(simpletextinput)).changed(actions) {
            log!("TEXTBOX CHANGED {}", self.counter); // output to console
            self.counter += 1;
            let lbl = self.ui.label(id!(simpletextinput_outputbox));
            lbl.set_text(cx, &format!("{} {}", self.counter, txt));
        }

        if self.ui.button(id!(basicbutton)).clicked(actions) {
            log!("BASIC BUTTON CLICKED {}", self.counter);
            self.counter += 1;
            let btn = self.ui.button(id!(basicbutton));
            btn.set_text(cx, &format!("Clicky clicky! {}", self.counter));
        }

        if self.ui.button(id!(styledbutton)).clicked(actions) {
            log!("STYLED BUTTON CLICKED {}", self.counter);
            self.counter += 1;
            let btn = self.ui.button(id!(styledbutton));
            btn.set_text(cx, &format!("Styled button clicked: {}", self.counter));
        }

        if self.ui.button(id!(iconbutton)).clicked(actions) {
            log!("ICON BUTTON CLICKED {}", self.counter);
            self.counter += 1;
            let btn = self.ui.button(id!(iconbutton));
            btn.set_text(cx, &format!("Icon button clicked: {}", self.counter));
        }

        if let Some(selected) = self.ui.drop_down(id!(dropdown)).changed(actions) {
            log!("DROPDOWN SELECTED index {}", selected);
            let selected_label = self.ui.drop_down(id!(dropdown)).selected_label();
            log!("DROPDOWN SELECTED label {}", selected_label);
        }
    }
}

impl AppMain for App {
    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());
    }
}

app_main!(App);
