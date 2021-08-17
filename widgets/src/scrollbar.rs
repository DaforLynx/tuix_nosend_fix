use crate::common::*;
use crate::Button;
use crate::scroll_container::Scroll;

pub enum ScrollDirection {
    Horizontal,
    Vertical,
}

pub struct Scrollbar {
    front: Entity,
    scroll: f32,

    pub position: f32,
    pub pos_ratio: f32,

    pressed_x: f32,
    pressed_y: f32,
    moving: bool,
    //on_scroll: Option<Box<dyn Fn(f32) -> Message>>,
}

impl Scrollbar {
    pub fn new() -> Self {
        Scrollbar {
            front: Entity::null(),
            scroll: 0.0,

            position: 0.0,
            pos_ratio: 0.2,

            pressed_x: 0.0,
            pressed_y: 0.0,
            moving: false,
            //on_scroll: None,
        }
    }

    pub fn set_posx(&self, state: &mut State, value: f32) {
        //self.back.set_left(state, value);
        self.front.set_left(state, Units::Pixels(value));
    }

    // pub fn on_scroll<F>(mut self, pos: F) -> Self
    // where
    //     F: 'static + Fn(f32) -> Message,
    // {
    //     self.on_scroll = Some(Box::new(pos));
    //     self
    // }
}

impl Widget for Scrollbar {
    type Ret = Entity;
    type Data = Scroll;
    fn on_build(&mut self, state: &mut State, entity: Entity) -> Self::Ret {
        self.front = Button::new().build(state, entity, |builder| 
            builder
                .set_background_color(Color::rgb(50, 50, 100))
                .class("front")
        );


        self.front
            .set_width(state, Units::Percentage(100.0))
            .set_height(state, Units::Percentage(100.0));
    


        entity.set_element(state, "scrollbar")
    }

    fn on_update(&mut self, state: &mut State, entity: Entity, data: &Self::Data) {
        self.scroll = data.scroll;
        entity.set_top(state, Percentage(self.scroll * data.overflow * 100.0));
    }

    fn on_event(&mut self, state: &mut State, entity: Entity, event: &mut Event) {
        /*
        if let Some(layout_event) = event.message.downcast::<LayoutEvent>() {
            match layout_event {
                LayoutEvent::Relayout => {
                    if *id != entity {
                        println!("LAYOUT EVENT");
                        let scroll = state
                        .style
                        .scroll
                        .get(self.entity)
                        .cloned()
                        .unwrap_or_default();
                        self.front
                            .set_top(state, Units::Percentage(scroll.y * (1.0 - scroll.h)));
                        self.front.set_height(state, Units::Percentage(scroll.h));

                        if scroll.h == 1.0 {
                            state.style.enabled.set(entity, false);
                        } else {
                            state.style.enabled.set(entity, true);
                        }
                        state.insert_event(
                            Event::new(StyleEvent::Restyle).target(state.root),
                        );
                        state.insert_event(
                            Event::new(LayoutEvent::Relayout).target(entity),
                        );

                    }
                }
            }
        }
        */

        /*
        if let Some(window_event) = event.message.downcast::<WindowEvent>() {
            match window_event {
                // When a relayout occurs, determine the new height of the scroll bar
                WindowEvent::Relayout => {}

                WindowEvent::WindowResize(_, _) => {
                    let scroll = state
                        .style
                        .scroll
                        .get(self.entity)
                        .cloned()
                        .unwrap_or_default();
                    self.front
                        .set_top(state, Units::Percentage(scroll.y * (1.0 - scroll.h)));
                    self.front.set_height(state, Units::Percentage(scroll.h));

                    if scroll.h == 1.0 {
                        //state.style.enabled.set(entity, false);
                        entity.set_disabled(state, true);
                    } else {
                        //state.style.enabled.set(entity, true);
                        entity.set_disabled(state, false);
                    }
                    state.insert_event(Event::new(WindowEvent::Restyle).target(Entity::root()));
                }

                WindowEvent::MouseScroll(_, y) => {
                    //scroll.y += (10.0 * y);
                    if event.target == entity || event.target == self.front {
                        if let Some(scroll) = state.style.scroll.get_mut(self.entity) {
                            scroll.y -= 0.1 * *y;

                            if scroll.y < 0.0 {
                                scroll.y = 0.0;
                            }

                            if scroll.y > 1.0 {
                                scroll.y = 1.0;
                            }
                        }

                        let scroll = state
                            .style
                            .scroll
                            .get(self.entity)
                            .cloned()
                            .unwrap_or_default();
                        self.front
                            .set_top(state, Units::Percentage(scroll.y * (1.0 - scroll.h)));
                        self.front.set_height(state, Units::Percentage(scroll.h));

                        if scroll.h == 1.0 {
                            //state.style.enabled.set(entity, false);
                            entity.set_disabled(state, true);
                        } else {
                            //state.style.enabled.set(entity, true);
                            entity.set_disabled(state, false);
                        }

                        state.insert_event(Event::new(WindowEvent::Restyle).target(Entity::root()));
                        state
                            .insert_event(Event::new(WindowEvent::Relayout).target(Entity::root()));
                        println!(
                            "Scroll: {}",
                            state
                                .style
                                .scroll
                                .get(self.entity)
                                .cloned()
                                .unwrap_or_default()
                                .y
                        );
                    }
                    //println!("y: {}", y);

                    //println!("Size: {}", state.data.get_height(self.front));
                }

                WindowEvent::MouseDown(button) => match button {
                    MouseButton::Left => {
                        self.pressed_x = state.mouse.cursorx;
                        self.pressed_y = state.mouse.cursory;
                        self.moving = true;
                        let scroll = state
                            .style
                            .scroll
                            .get(self.entity)
                            .cloned()
                            .unwrap_or_default();
                        self.position = scroll.y;
                        state.capture(entity);
                    }
                    _ => {}
                },

                WindowEvent::MouseUp(button) => match button {
                    MouseButton::Left => {
                        self.moving = false;
                        state.release(entity);
                    }

                    _ => {}
                },

                WindowEvent::MouseMove(_, y) => {
                    if self.moving {
                        let dist_y = *y - self.pressed_y;
                        let overflow =
                            state.data.get_height(entity) - state.data.get_height(self.front);
                        let ratio = dist_y / overflow;
                        let r = self.position + ratio;
                        if let Some(scroll) = state.style.scroll.get_mut(self.entity) {
                            scroll.y = r;

                            if scroll.y < 0.0 {
                                scroll.y = 0.0;
                            }

                            if scroll.y > 1.0 {
                                scroll.y = 1.0;
                            }
                        }

                        let scroll = state
                            .style
                            .scroll
                            .get(self.entity)
                            .cloned()
                            .unwrap_or_default();
                        self.front
                            .set_top(state, Units::Percentage(scroll.y * (1.0 - scroll.h)));

                        state.insert_event(Event::new(WindowEvent::Restyle).target(Entity::root()));
                        state
                            .insert_event(Event::new(WindowEvent::Relayout).target(Entity::root()));
                        //println!("overflow: {}, dist: {}, ratio: {}", overflow, dist_y, r);
                    }
                }

                _ => {}
            }
        }
        */
    }
}
