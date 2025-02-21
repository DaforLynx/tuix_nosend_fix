#![allow(dead_code)]

use crate::entity::Entity;
use crate::events::*;
use crate::mouse::*;
use crate::{BuildHandler, Justify, Length, PropSet, State, Visibility, WindowEvent};

use glutin::event::VirtualKeyCode;

use femtovg::{
    renderer::OpenGl, Align, Baseline, Canvas, Color, FillRule, FontId, ImageFlags, ImageId,
    LineCap, LineJoin, Paint, Path, Renderer, Solidity,
};

use crate::Key;

#[derive(Debug, Clone, PartialEq)]
pub enum TextboxEvent {
    SetValue(String),
    ValueChanged(String),
    ResetValue,
}

//impl Message for TextboxEvent {}

#[derive(Clone)]
pub struct Textbox {
    entity: Entity,
    text: String,

    buffer: String,

    units: String,
    multiplier: f32,

    select_pos: u32,
    cursor_pos: u32,
    edit: bool,
    hitx: f32,
    dragx: f32,
}

impl Textbox {
    pub fn new(text: &str) -> Self {
        // id.set_text(state, "Test".to_string())
        //     .set_background(state, nanovg::Color::from_rgb(100, 50, 50));

        Textbox {
            entity: Entity::null(),

            text: text.to_string(),

            buffer: String::new(),

            units: String::new(),

            multiplier: 1.0,

            select_pos: 0,
            cursor_pos: 0,
            edit: false,
            hitx: -1.0,
            dragx: -1.0,
        }
    }

    
    pub fn with_units(mut self, uints: &str) -> Self {

        self.units = uints.to_string();

        self
    }

    // pub fn set_enabled(&self, state: &mut WidgetState, val: bool) {
    //     if val {
    //         self.id
    //             .set_background(state, nanovg::Color::from_rgb(100, 50, 50));
    //     } else {
    //         self.id
    //             .set_background(state, nanovg::Color::from_rgb(50, 50, 100));
    //     }
    // }
}

impl BuildHandler for Textbox {
    type Ret = Entity;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        entity.set_text(state, &(self.text.to_owned() + &self.units));


        self.entity = entity;

        state.style.insert_element(entity, "textbox");

        entity
    }
}

impl EventHandler for Textbox {
    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) -> bool {
        if let Some(textbox_event) = event.message.downcast::<TextboxEvent>() {
            match textbox_event {
                TextboxEvent::SetValue(val) => {
                    if event.target == entity {
                        entity.set_text(state, &(val.to_owned() + &self.units));

                        // state.insert_event(
                        //     Event::new(WindowEvent::Restyle).target(Entity::new(0, 0)),
                        // );

                        state.insert_event(Event::new(WindowEvent::Redraw));
                    }
                }

                // TextboxEvent::ResetValue => {
                //     if let Some(text_data) = state.style.text.get_mut(entity) {
                //         text_data.text = self.buffer.clone();
                //     }
                // }
                _ => {}
            }
        }

        let text_data = state.style.text.get(entity).cloned().unwrap_or_default();

        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                WindowEvent::MouseMove(x, _) => {
                    if self.hitx != -1.0 {
                        self.dragx = *x;

                        // state.insert_event(
                        //     Event::new(WindowEvent::Restyle).target(Entity::new(0, 0)),
                        // );

                        state.insert_event(Event::new(WindowEvent::Redraw));
                    }
                }

                WindowEvent::MouseDown(button) => {
                    if entity == state.hovered {
                        if self.edit == false && !entity.is_disabled(state) {
                            self.cursor_pos = text_data.text.len() as u32;
                            self.select_pos = 0;
                            self.buffer = text_data.text.clone();
                            state.focused = entity;
                            //state.captured = entity;
                            state.capture(entity);
                            self.edit = true;
                            entity.set_active(state, true);
                        }
                        if self.edit == true {
                            self.hitx = state.mouse.cursorx;
                            self.dragx = state.mouse.cursorx;
                        }
                        //self.edit = true;

                        

                        // state.insert_event(
                        //     Event::new(WindowEvent::Restyle).target(Entity::new(0, 0)),
                        // );

                        state.insert_event(Event::new(WindowEvent::Redraw));
                    } else {
                        self.edit = false;
                        entity.set_active(state, false);

                        state.insert_event(
                            Event::new(TextboxEvent::ValueChanged(text_data.text.clone()))
                                .target(entity),
                        );

                        // state.insert_event(
                        //     Event::new(WindowEvent::Restyle).target(Entity::new(0, 0)),
                        // );

                        state.insert_event(Event::new(WindowEvent::Redraw));

                        if state.captured == entity {
                            state.insert_event(
                                Event::new(WindowEvent::MouseDown(*button)).target(state.hovered),
                            );
                        }

                        if state.focused == entity {
                            state.focused = Entity::new(0, 0);
                        }

                        //state.captured = Entity::null();
                        state.release(entity);
                    }
                }

                WindowEvent::MouseUp(_) => {
                    self.hitx = -1.0;
                }

                WindowEvent::KeyDown(code, key) => {
                    println!("Code: {:?} Key: {:?}", code, key);
                    if *key == Some(Key::ArrowLeft) {
                        
                        if self.edit {
                            self.hitx = -1.0;
                            if self.cursor_pos > 0 {
                                self.cursor_pos -= 1;
                            }
                            if !state.modifiers.shift {
                                self.select_pos = self.cursor_pos;
                            }

                            // state.insert_event(
                            //     Event::new(WindowEvent::Restyle).target(Entity::new(0, 0)),
                            // );

                            state.insert_event(Event::new(WindowEvent::Redraw));
                        }
                    }

                    if *key == Some(Key::ArrowRight) {
                        if self.edit {
                            self.hitx = -1.0;
                            if self.cursor_pos < text_data.text.len() as u32 {
                                self.cursor_pos += 1;
                            }
                            if !state.modifiers.shift {
                                self.select_pos = self.cursor_pos;
                            }

                            // state.insert_event(
                            //     Event::new(WindowEvent::Restyle).target(Entity::new(0, 0)),
                            // );

                            state.insert_event(Event::new(WindowEvent::Redraw));
                        }
                    }
                    if *key == Some(Key::Backspace) {
                        if self.edit {
                            let start =
                                std::cmp::min(self.select_pos, self.cursor_pos) as usize;
                            let end = std::cmp::max(self.select_pos, self.cursor_pos) as usize;
                            //let start = text_data.select_pos as usize;
                            //let end = text_data.cursor_pos as usize;
                            if start == end && self.cursor_pos > 0 {
                                if let Some(txt) = state.style.text.get_mut(entity) {
                                    txt.text.remove((self.cursor_pos - 1) as usize);
                                }

                                self.cursor_pos -= 1;
                                self.select_pos -= 1;
                            } else {
                                if let Some(txt) = state.style.text.get_mut(entity) {
                                    txt.text.replace_range(start..end, "");
                                }
                                self.cursor_pos = start as u32;
                                self.select_pos = start as u32;
                            }

                            // state.insert_event(
                            //     Event::new(WindowEvent::Restyle).target(Entity::new(0, 0)),
                            // );

                            state.insert_event(Event::new(WindowEvent::Redraw));
                        }
                    }
                    if *key == Some(Key::Enter) {
                        if self.edit {
                            //text_data.buffer = text_data.text.clone();
                            state.insert_event(
                                Event::new(TextboxEvent::ValueChanged(text_data.text.clone()))
                                    .target(entity),
                            );

                            self.edit = false;
                            entity.set_active(state, false);
                            state.focused = Entity::new(0, 0);
                            state.captured = Entity::null();

                            // state.insert_event(
                            //     Event::new(WindowEvent::Restyle).target(Entity::new(0, 0)),
                            // );

                            state.insert_event(Event::new(WindowEvent::Redraw));
                        }
                    }
                    if *key == Some(Key::Escape) {
                        if self.edit {
                            self.text = self.buffer.clone();
                            self.edit = false;
                            entity.set_active(state, false);

                            // state.insert_event(
                            //     Event::new(WindowEvent::Restyle).target(Entity::new(0, 0)),
                            // );

                            state.insert_event(Event::new(WindowEvent::Redraw));
                        }
                    }
                
                }

                WindowEvent::CharInput(input) => {
                    if *input as u8 != 8 && *input as u8 != 13 {
                        if self.edit {
                            let start = std::cmp::min(self.select_pos, self.cursor_pos) as usize;
                            let end = std::cmp::max(self.select_pos, self.cursor_pos) as usize;
                            //let start = text_data.select_pos as usize;
                            //let end = text_data.cursor_pos as usize;
                            if start == end {
                                if let Some(txt) = state.style.text.get_mut(entity) {
                                    txt.text.insert(start, *input);
                                }

                                //text_data.text.remove((text_data.cursor_pos - 1) as usize);
                                self.cursor_pos += 1;
                                self.select_pos += 1;
                            } else {
                                if let Some(txt) = state.style.text.get_mut(entity) {
                                    txt.text.replace_range(start..end, &input.to_string());
                                }
                                self.cursor_pos = (start + 1) as u32;
                                self.select_pos = (start + 1) as u32;
                            }

                            // state.insert_event(
                            //     Event::new(WindowEvent::Restyle).target(Entity::new(0, 0)),
                            // );

                            state.insert_event(Event::new(WindowEvent::Redraw));
                        }
                    }
                }

                _ => {}
            }
        }

        false
    }

    fn on_draw(
        &mut self,
        state: &mut State,
        entity: Entity,
        canvas: &mut Canvas<OpenGl>,
        //images: &HashMap<String, nanovg::Image>,
    ) {
        // Skip window
        if entity == Entity::new(0, 0) {
            return;
        }

        // Skip invisible widgets
        if state.transform.get_visibility(entity) == Visibility::Invisible {
            return;
        }

        if state.transform.get_opacity(entity) == 0.0 {
            return;
        }

        let posx = state.transform.get_posx(entity);
        let posy = state.transform.get_posy(entity);
        let width = state.transform.get_width(entity);
        let height = state.transform.get_height(entity);

        //println!("entity: {} posx: {} posy: {} width: {} height: {}", entity, posx, posy, width, height);



        let padding_left = match state
            .style
            .padding_left
            .get(entity)
            .unwrap_or(&Length::Auto)
        {
            Length::Pixels(val) => val,
            _ => &0.0,
        };

        let padding_right = match state
            .style
            .padding_right
            .get(entity)
            .unwrap_or(&Length::Auto)
        {
            Length::Pixels(val) => val,
            _ => &0.0,
        };

        let padding_top = match state.style.padding_top.get(entity).unwrap_or(&Length::Auto) {
            Length::Pixels(val) => val,
            _ => &0.0,
        };

        let padding_bottom = match state
            .style
            .padding_bottom
            .get(entity)
            .unwrap_or(&Length::Auto)
        {
            Length::Pixels(val) => val,
            _ => &0.0,
        };



        let background_color = state
            .style
            .background_color
            .get(entity)
            .cloned()
            .unwrap_or_default();

        let font_color = state
            .style
            .font_color
            .get(entity)
            .cloned()
            .unwrap_or(crate::Color::rgb(255, 255, 255));

        let border_color = state
            .style
            .border_color
            .get(entity)
            .cloned()
            .unwrap_or_default();

        let shadow_color = state
            .style
            .shadow_color
            .get(entity)
            .cloned()
            .unwrap_or_default();

        let parent = state
            .hierarchy
            .get_parent(entity)
            .expect("Failed to find parent somehow");

        let parent_width = state.transform.get_width(parent);

        let border_radius_top_left = match state.style.border_radius_top_left.get(entity).cloned().unwrap_or_default() {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let border_radius_top_right = match state.style.border_radius_top_right.get(entity).cloned().unwrap_or_default() {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let border_radius_bottom_left = match state.style.border_radius_bottom_left.get(entity).cloned().unwrap_or_default() {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let border_radius_bottom_right = match state.style.border_radius_bottom_right.get(entity).cloned().unwrap_or_default() {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        let opacity = state.transform.get_opacity(entity);

        let mut background_color: femtovg::Color = background_color.into();
        background_color.set_alphaf(background_color.a * opacity);

        let mut border_color: femtovg::Color = border_color.into();
        border_color.set_alphaf(border_color.a * opacity);

        let mut shadow_color: femtovg::Color = shadow_color.into();
        shadow_color.set_alphaf(shadow_color.a * opacity);

        let border_width = match state
            .style
            .border_width
            .get(entity)
            .cloned()
            .unwrap_or_default() 
        {
            Length::Pixels(val) => val,
            Length::Percentage(val) => parent_width * val,
            _ => 0.0,
        };

        //println!("Border Width: {}", border_width);

        
        // Skip widgets with no width or no height
        if width + 2.0 * border_width + padding_left + padding_right == 0.0 || height + 2.0 * border_width + padding_top + padding_bottom == 0.0 {
            return;
        }

        
        
        // Apply transformations
        let rotate = state.style.rotate.get(entity).unwrap_or(&0.0);
        let scaley = state.style.scaley.get(entity).cloned().unwrap_or_default();

        canvas.save();
        // canvas.translate(posx + width / 2.0, posy + height / 2.0);
        // canvas.rotate(rotate.to_radians());
        // canvas.translate(-(posx + width / 2.0), -(posy + height / 2.0));

        let pt = canvas.transform().inverse().transform_point(posx + width / 2.0, posy + height / 2.0);
        //canvas.translate(posx + width / 2.0, posy + width / 2.0);
        canvas.translate(pt.0, pt.1);
        canvas.scale(1.0, scaley.0);
        canvas.translate(-pt.0, -pt.1);


        // Apply Scissor
        let clip_entity = state.transform.get_clip_widget(entity);

        let clip_posx = state.transform.get_posx(clip_entity);
        let clip_posy = state.transform.get_posy(clip_entity);
        let clip_width = state.transform.get_width(clip_entity);
        let clip_height = state.transform.get_height(clip_entity);

        canvas.scissor(clip_posx, clip_posy, clip_width, clip_height);


        
        let shadow_h_offset = state
            .style
            .shadow_h_offset
            .get(entity)
            .cloned()
            .unwrap_or_default();

        // Draw shadow
        // let mut path = Path::new();
        // path.rounded_rect_varying(posx, posy, width, height, border_radius_top_left, border_radius_top_right, border_radius_bottom_right, border_radius_bottom_left);
        // let mut paint = Paint::color(background_color);
        // canvas.fill_path(&mut path, paint);

        // Draw rounded rect
        let mut path = Path::new();
        path.rounded_rect_varying(
            posx + (border_width / 2.0),
            posy + (border_width / 2.0),
            width - border_width,
            height - border_width,
            border_radius_top_left,
            border_radius_top_right,
            border_radius_bottom_right,
            border_radius_bottom_left,
        );
        let mut paint = Paint::color(background_color);
        canvas.fill_path(&mut path, &paint);

        // Draw border
        let mut paint = Paint::color(border_color);
        paint.set_line_width(border_width);
        //paint.set_anti_alias(false);
        canvas.stroke_path(&mut path, &paint);
        //println!("posx: {}", posx);

        let mut font_color: femtovg::Color = font_color.into();
        font_color.set_alphaf(font_color.a * opacity);


        if let Some(text) = state.style.text.get_mut(entity) {
            let font_id = match text.font.as_ref() {
                "Sans" => state.fonts.regular.unwrap(),
                "Icons" => state.fonts.icons.unwrap(),
                _ => state.fonts.regular.unwrap(),
            };

            let mut x = posx;
            let mut y = posy;

            let text_string = text.text.to_owned();

            let text_align = state
                .style
                .text_align
                .get(entity)
                .cloned()
                .unwrap_or_default();
            let text_justify = state
                .style
                .text_justify
                .get(entity)
                .cloned()
                .unwrap_or_default();

            let align = match text_justify {
                Justify::Start => {
                    x += padding_left;
                    Align::Left
                }
                Justify::Center => {
                    x += 0.5 * width;
                    Align::Center
                }
                Justify::End => {
                    x += width - padding_right;
                    Align::Right
                }
            };

            let baseline = match text_align {
                crate::Align::Start => {
                    y += padding_top;
                    Baseline::Top
                }
                crate::Align::Center => {
                    y += 0.5 * height;
                    Baseline::Middle
                }
                crate::Align::End => {
                    y += height - padding_bottom;
                    Baseline::Bottom
                }
            };

            let font_size = state.style.font_size.get(entity).cloned().unwrap_or(16.0);

            let mut paint = Paint::color(font_color);
            paint.set_font_size(font_size);
            paint.set_font(&[font_id]);
            paint.set_text_align(align);
            paint.set_text_baseline(baseline);

            if let Ok(res) = canvas.fill_text(x, y, &text_string, &paint) {
                let text_width = res.width();
                let mut glyph_positions = res.glyphs.iter().peekable();

                let mut caretx = posx + padding_left;

                let mut selectx = caretx;

                if self.edit {
                    let startx = x - text_width / 2.0;
                    let endx = x + text_width / 2.0;
                    if self.hitx != -1.0 {

                        //let endx = res.glyphs.last().unwrap().x + res.glyphs.last().unwrap().w;

                        selectx = if self.hitx < startx + text_width / 2.0 {
                            self.select_pos = 0;
                            startx
                        } else {
                            self.select_pos = text.text.len() as u32;
                            endx
                        };

                        caretx = if self.dragx < startx + text_width / 2.0 {
                            self.cursor_pos = 0;
                            startx
                        } else {
                            self.cursor_pos = text.text.len() as u32;
                            endx
                        };

                        let mut n = 0;
                        let mut px = x + padding_left;

                        for glyph in res.glyphs.iter() {
                            let left_edge = glyph.x;
                            let right_edge = left_edge + glyph.width;
                            let gx = left_edge * 0.3 + right_edge * 0.7;

                            // if n == 0 && self.hitx <= glyph.x {
                            //     selectx = left_edge;
                            //     self.select_pos = 0;
                            // }

                            // if n == res.glyphs.len() as u32 && self.hitx >= glyph.x + glyph.width {
                            //     selectx = right_edge;
                            //     self.select_pos = n;
                            // }

                            // if n == 0 && self.dragx <= glyph.x {
                            //     caretx = left_edge;
                            //     self.cursor_pos = 0;
                            // }

                            // if n == res.glyphs.len() as u32 && self.hitx >= glyph.x + glyph.width {
                            //     caretx = right_edge;
                            //     self.cursor_pos = n;
                            // }



                            if self.hitx >= px && self.hitx < gx {
                                selectx = left_edge;

                                self.select_pos = n;
                            }

                            if self.dragx >= px && self.dragx < gx {
                                caretx = left_edge;

                                self.cursor_pos = n;
                            }

                            px = gx;
                            n += 1;
                        }
                    } else {
                        let mut n = 0;

                        //let mut start_x = 0.0;

                        for glyph in res.glyphs.iter() {

                            if n == self.cursor_pos {
                                caretx = glyph.x;
                            }

                            if n == self.select_pos {
                                selectx = glyph.x;
                            }

                            n += 1;
                        }

                        if self.cursor_pos as usize == text.text.len() {
                            caretx = endx;
                        }

                        if self.select_pos as usize == text.text.len() {
                            selectx = endx;
                        }
                    }

                    //Draw selection
                    let select_width = (caretx - selectx).abs();
                    if selectx > caretx {
                        let mut path = Path::new();
                        path.rect(caretx, y - 1.2 * res.height()/2.0, select_width, 1.3*res.height());
                        canvas.fill_path(&mut path, &Paint::color(Color::rgba(0, 0, 0, 64)));
                    } else if caretx > selectx {
                        let mut path = Path::new();
                        path.rect(selectx, y - 1.2 * res.height()/2.0, select_width, 1.3*res.height());
                        canvas.fill_path(&mut path, &Paint::color(Color::rgba(0, 0, 0, 64)));
                    }

                    let mut path = Path::new();
                    path.rect(caretx - 1.0, y - 1.2*res.height()/2.0, 2.0, 1.3*res.height());
                    canvas.fill_path(&mut path, &Paint::color(Color::rgba(247, 76, 0, 255)));

                    // let mut path = Path::new();
                    // path.rect(endx, y - 0.25 * height, 1.0, height * 0.5);
                    // canvas.fill_path(&mut path, Paint::color(Color::rgba(255, 0, 0, 255)));
                }
            }
        }
    }
}
