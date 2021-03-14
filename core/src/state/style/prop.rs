use crate::state::style::*;
use crate::State;
use crate::{entity::Entity, BuildHandler, Builder, EventHandler, Propagation};

use crate::{Event, WindowEvent};

use crate::state::hierarchy::*;

pub trait PropSet {

    /// Add a class name to an entity
    fn class(self, state: &mut State, class_name: &str) -> Self;

    // TODO move to PropGet
    fn get_parent(self, state: &mut State) -> Option<Entity>;

    // Pseudoclass
    fn set_enabled(self, state: &mut State, value: bool) -> Self;
    fn set_disabled(self, state: &mut State, value: bool) -> Self;
    fn set_checked(self, state: &mut State, value: bool) -> Self;
    fn set_over(self, state: &mut State, value: bool) -> Self;
    fn set_active(self, state: &mut State, value: bool) -> Self;
    fn set_hover(self, state: &mut State, value: bool) -> Self;
    fn set_focus(self, state: &mut State, value: bool) -> Self;

    // Style
    fn set_element(self, state: &mut State, value: &str) -> Self;
    fn set_id(self, state: &mut State, value: &str) -> Self;
    fn set_class(self, state: &mut State, value: &str) -> Self;

    // Visibility
    fn set_visibility(self, state: &mut State, value: Visibility) -> Self;

    // Overflow
    fn set_overflow(self, state: &mut State, value: Overflow) -> Self;

    // Display
    fn set_display(self, state: &mut State, value: Display) -> Self;

    //Opacity
    fn set_opacity(self, state: &mut State, value: f32) -> Self;

    // Rotate
    fn set_rotate(self, state: &mut State, value: f32) -> Self;

    // Grid Container
    //fn set_grid_columns(self, state: &mut State, value: Vec<f32>) -> Self;
    //fn set_grid_rows(self, state: &mut State, value: Vec<f32>) -> Self;

    // Grid Item
    //fn set_grid_column_start(self, state: &mut State, value: u32) -> Self;
    //fn set_grid_column_span(self, state: &mut State, value: u32) -> Self;

    // Flex Container
    fn set_flex_direction(self, state: &mut State, value: FlexDirection) -> Self;
    fn set_justify_content(self, state: &mut State, value: JustifyContent) -> Self;
    fn set_align_content(self, state: &mut State, value: AlignContent) -> Self;
    fn set_align_items(self, state: &mut State, value: AlignItems) -> Self;

    // Flex Item
    fn set_flex_grow(self, state: &mut State, value: f32) -> Self;
    fn set_flex_shrink(self, state: &mut State, value: f32) -> Self;
    fn set_flex_basis(self, state: &mut State, value: Length) -> Self;
    fn set_align_self(self, state: &mut State, value: AlignSelf) -> Self;

    // Positioning
    fn set_position(self, state: &mut State, value: Position) -> Self;
    fn set_left(self, state: &mut State, value: Length) -> Self;
    fn set_right(self, state: &mut State, value: Length) -> Self;
    fn set_top(self, state: &mut State, value: Length) -> Self;
    fn set_bottom(self, state: &mut State, value: Length) -> Self;

    // Size
    fn set_width(self, state: &mut State, value: Length) -> Self;
    fn set_height(self, state: &mut State, value: Length) -> Self;

    // Size Constraints
    fn set_min_width(self, state: &mut State, value: Length) -> Self;
    fn set_max_width(self, state: &mut State, value: Length) -> Self;
    fn set_min_height(self, state: &mut State, value: Length) -> Self;
    fn set_max_height(self, state: &mut State, value: Length) -> Self;

    // Text
    fn set_text(self, state: &mut State, text: &str) -> Self;

    // Text Font
    fn set_font(self, state: &mut State, font: &str) -> Self;
    fn set_font_size(self, state: &mut State, size: f32) -> Self;
    fn set_font_color(self, state: &mut State, color: Color) -> Self;

    // Text Alignment
    fn set_text_align(self, state: &mut State, align: Align) -> Self;
    fn set_text_justify(self, state: &mut State, justify: Justify) -> Self;

    // Tooltip
    fn set_tooltip(self, state: &mut State, text: &str) -> Self;

    // Background
    fn set_background_color(self, state: &mut State, value: Color) -> Self;
    fn set_background_image(self, state: &mut State, value: String) -> Self;

    // Border
    fn set_border_width(self, state: &mut State, value: Length) -> Self;
    fn set_border_color(self, state: &mut State, value: Color) -> Self;

    // Border Radius
    fn set_border_radius(self, state: &mut State, value: Length) -> Self;
    fn set_border_radius_top_left(self, state: &mut State, value: Length) -> Self;
    fn set_border_radius_top_right(self, state: &mut State, value: Length) -> Self;
    fn set_border_radius_bottom_left(self, state: &mut State, value: Length) -> Self;
    fn set_border_radius_bottom_right(self, state: &mut State, value: Length) -> Self;

    // Margin
    fn set_margin(self, state: &mut State, value: Length) -> Self;
    fn set_margin_left(self, state: &mut State, value: Length) -> Self;
    fn set_margin_right(self, state: &mut State, value: Length) -> Self;
    fn set_margin_top(self, state: &mut State, value: Length) -> Self;
    fn set_margin_bottom(self, state: &mut State, value: Length) -> Self;

    // Padding
    fn set_padding(self, state: &mut State, value: Length) -> Self;
    fn set_padding_left(self, state: &mut State, value: Length) -> Self;
    fn set_padding_right(self, state: &mut State, value: Length) -> Self;
    fn set_padding_top(self, state: &mut State, value: Length) -> Self;
    fn set_padding_bottom(self, state: &mut State, value: Length) -> Self;

    // Clipping
    fn set_clip_widget(self, state: &mut State, value: Entity) -> Self;

    fn set_z_order(self, state: &mut State, vaale: i32) -> Self;

    fn set_next_focus(self, state: &mut State, value: Entity) -> Self;
    fn set_prev_focus(self, state: &mut State, value: Entity) -> Self;
    fn set_focus_order(self, state: &mut State, next: Entity, prev: Entity) -> Self;

    fn mutate<F: FnMut(Builder) -> Builder>(self, state: &mut State, builder: F) -> Self;

    fn testy<B: EventHandler + 'static>(self, state: &mut State) -> Option<&mut B>;

    fn testy2<B: EventHandler + 'static, F: FnMut(&mut B)>(
        self,
        state: &mut State,
        mutator: F,
    ) -> Self;
}

impl PropSet for Entity {
    fn testy<B: EventHandler + 'static>(self, state: &mut State) -> Option<&mut B>
    where
        Self: std::marker::Sized + 'static,
    {
        let t = state.event_handlers.get_mut(&self).unwrap();

        let t1 = t.downcast::<B>();

        t1
    }

    fn testy2<B: EventHandler + 'static, F: FnMut(&mut B)>(
        self,
        state: &mut State,
        mut mutator: F,
    ) -> Self
    where
        Self: std::marker::Sized + 'static,
    {
        let t = state.event_handlers.get_mut(&self).unwrap();

        let t1 = t.downcast::<B>().expect("Failed to cast");

        mutator(t1);

        self
    }

    fn mutate<F>(self, state: &mut State, mut builder: F) -> Self
    where
        F: FnMut(Builder) -> Builder,
    {
        builder(Builder::new(state, self));

        self
    }

    fn class(self, state: &mut State, class_name: &str) -> Self {
        state.style.insert_class(self, class_name);

        self
    }

    fn get_parent(self, state: &mut State) -> Option<Entity> {
        self.parent(&state.hierarchy)
    }

    // PseudoClass
    fn set_enabled(self, state: &mut State, value: bool) -> Self {
        if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(self) {
            pseudo_classes.set_enabled(value);
            pseudo_classes.set_disabled(!value);
        }

        state.needs_restyle = true;

        self
    }

    fn set_disabled(self, state: &mut State, value: bool) -> Self {
        if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(self) {
            pseudo_classes.set_disabled(value);
            pseudo_classes.set_enabled(!value);
        }

        state.needs_restyle = true;

        self
    }

    fn set_checked(self, state: &mut State, value: bool) -> Self {
        if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(self) {
            pseudo_classes.set_checked(value);
        }

        state.needs_restyle = true;
        state.needs_relayout = true;
        state.needs_redraw = true;

        self
    }

    fn set_over(self, state: &mut State, value: bool) -> Self {
        if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(self) {
            pseudo_classes.set_over(value);
        }

        state.needs_restyle = true;

        self
    }

    fn set_active(self, state: &mut State, value: bool) -> Self {
        if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(self) {
            pseudo_classes.set_active(value);
        }

        state.needs_restyle = true;

        self
    }

    fn set_hover(self, state: &mut State, value: bool) -> Self {
        if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(self) {
            pseudo_classes.set_hover(value);
        }

        state.needs_restyle = true;

        self
    }

    fn set_focus(self, state: &mut State, value: bool) -> Self {
        if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(self) {
            pseudo_classes.set_focus(value);
        }

        state.needs_restyle = true;

        self
    }

    // Style
    fn set_element(self, state: &mut State, value: &str) -> Self {
        state.style.insert_element(self, value);

        self
    }

    fn set_id(self, state: &mut State, value: &str) -> Self {
        state.style.insert_id(self, value);

        self
    }

    fn set_class(self, state: &mut State, value: &str) -> Self {
        state.style.insert_class(self, value);

        self
    }

    // Visibility
    fn set_visibility(self, state: &mut State, value: Visibility) -> Self {
        state.style.visibility.insert(self, value);

        state.needs_relayout = true;
        state.needs_redraw = true;

        self
    }

    // Overflow
    fn set_overflow(self, state: &mut State, value: Overflow) -> Self {
        state.style.overflow.insert(self, value);


        state.needs_relayout = true;
        state.needs_redraw = true;

        self
    }

    // Display
    fn set_display(self, state: &mut State, value: Display) -> Self {
        state.style.display.insert(self, value);

        state.needs_relayout = true;
        state.needs_redraw = true;

        self
    }

    //Opacity
    fn set_opacity(self, state: &mut State, value: f32) -> Self {
        state.style.opacity.insert(self, Opacity(value));

        state.needs_relayout = true;
        state.needs_redraw = true;

        self
    }

    // Rotate
    fn set_rotate(self, state: &mut State, value: f32) -> Self {
        state.style.rotate.insert(self, value);

        state.needs_relayout = true;
        state.needs_redraw = true;

        self
    }

    // Flex Container
    fn set_flex_direction(self, state: &mut State, value: FlexDirection) -> Self {
        state.style.flex_direction.insert(self, value);

        state.needs_relayout = true;
        state.needs_redraw = true;

        self
    }

    //TODO
    // fn set_flex_wrap(self, state: &mut State, value: FlexDirection) -> Self {
    //     if let Some(data) = state.style.grid_container.get_mut(self) {
    //         data.flex_direction = value;
    //     }

    //     self
    // }

    fn set_justify_content(self, state: &mut State, value: JustifyContent) -> Self {
        state.style.justify_content.insert(self, value);

        state.needs_relayout = true;
        state.needs_redraw = true;

        self
    }

    fn set_align_content(self, state: &mut State, value: AlignContent) -> Self {
        state.style.align_content.insert(self, value);

        state.needs_relayout = true;
        state.needs_redraw = true;

        self
    }

    fn set_align_items(self, state: &mut State, value: AlignItems) -> Self {
        state.style.align_items.insert(self, value);

        state.needs_relayout = true;
        state.needs_redraw = true;

        self
    }

    // Flex Item
    fn set_flex_grow(self, state: &mut State, value: f32) -> Self {
        state.style.flex_grow.insert(self, value);

        state.needs_relayout = true;
        state.needs_redraw = true;

        self
    }

    fn set_flex_shrink(self, state: &mut State, value: f32) -> Self {
        state.style.flex_shrink.insert(self, value);

        state.needs_relayout = true;
        state.needs_redraw = true;

        self
    }

    fn set_flex_basis(self, state: &mut State, value: Length) -> Self {
        state.style.flex_basis.insert(self, value);

        state.needs_relayout = true;
        state.needs_redraw = true;

        self
    }

    fn set_align_self(self, state: &mut State, value: AlignSelf) -> Self {
        state.style.align_self.insert(self, value);

        state.needs_relayout = true;
        state.needs_redraw = true;

        self
    }

    // Positioning
    fn set_position(self, state: &mut State, value: Position) -> Self {
        state.style.position.insert(self, value);

        state.needs_relayout = true;
        state.needs_redraw = true;

        self
    }

    fn set_left(self, state: &mut State, value: Length) -> Self {
        state.style.left.insert(self, value);

        state.needs_relayout = true;
        state.needs_redraw = true;

        self
    }

    fn set_right(self, state: &mut State, value: Length) -> Self {
        state.style.right.insert(self, value);

        state.needs_relayout = true;
        state.needs_redraw = true;

        self
    }

    fn set_top(self, state: &mut State, value: Length) -> Self {
        state.style.top.insert(self, value);

        state.needs_relayout = true;
        state.needs_redraw = true;

        self
    }

    fn set_bottom(self, state: &mut State, value: Length) -> Self {
        state.style.bottom.insert(self, value);

        state.needs_relayout = true;
        state.needs_redraw = true;

        self
    }

    // Size
    fn set_width(self, state: &mut State, value: Length) -> Self {
        state.style.width.insert(self, value);

        state.needs_relayout = true;
        state.needs_redraw = true;

        self
    }

    fn set_height(self, state: &mut State, value: Length) -> Self {
        state.style.height.insert(self, value);

        state.needs_relayout = true;
        state.needs_redraw = true;

        self
    }

    // Size Constraints
    fn set_min_width(self, state: &mut State, value: Length) -> Self {
        state.style.min_width.insert(self, value);

        state.needs_relayout = true;
        state.needs_redraw = true;

        self
    }

    fn set_max_width(self, state: &mut State, value: Length) -> Self {
        state.style.max_width.insert(self, value);

        state.needs_relayout = true;
        state.needs_redraw = true;

        self
    }

    fn set_min_height(self, state: &mut State, value: Length) -> Self {
        state.style.min_height.insert(self, value);

        state.needs_relayout = true;
        state.needs_redraw = true;

        self
    }

    fn set_max_height(self, state: &mut State, value: Length) -> Self {
        state.style.max_height.insert(self, value);

        state.needs_relayout = true;
        state.needs_redraw = true;

        self
    }

    // Tooltip
    fn set_tooltip(self, state: &mut State, value: &str) -> Self {
        state.style.tooltip.insert(self, value.to_string());

        self
    }

    // Text
    fn set_text(self, state: &mut State, value: &str) -> Self {
        if let Some(data) = state.style.text.get_mut(self) {
            data.text = value.to_string();
        } else {
            state.style.text.insert(
                self,
                Text {
                    text: value.to_string(),
                    ..Default::default()
                },
            );
        }

        state.needs_relayout = true;
        state.needs_redraw = true;

        self
    }

    // Text Font
    fn set_font(self, state: &mut State, value: &str) -> Self {
        if let Some(data) = state.style.text.get_mut(self) {
            data.font = value.to_string();
        } else {
            state.style.text.insert(
                self,
                Text {
                    font: value.to_string(),
                    ..Default::default()
                },
            );
        }

        state.needs_relayout = true;
        state.needs_redraw = true;

        self
    }

    fn set_font_size(self, state: &mut State, value: f32) -> Self {
        state.style.font_size.insert(self, value);

        state.needs_relayout = true;
        state.needs_redraw = true;

        self
    }

    fn set_font_color(self, state: &mut State, value: Color) -> Self {
        state.style.font_color.insert(self, value);

        state.needs_redraw = true;

        self
    }

    // Text Alignment
    fn set_text_justify(self, state: &mut State, value: Justify) -> Self {
        state.style.text_justify.insert(self, value);

        state.needs_redraw = true;

        self
    }

    fn set_text_align(self, state: &mut State, value: Align) -> Self {
        state.style.text_align.insert(self, value);

        state.needs_redraw = true;

        self
    }

    // Background
    fn set_background_color(self, state: &mut State, value: Color) -> Self {
        state.style.background_color.insert(self, value);

        state.needs_redraw = true;

        self
    }

    fn set_background_image(self, state: &mut State, value: String) -> Self {
        state.style.background_image.insert(self, value);

        state.needs_redraw = true;

        self
    }

    // Border
    fn set_border_width(self, state: &mut State, value: Length) -> Self {
        state.style.border_width.insert(self, value);

        state.needs_relayout = true;
        state.needs_redraw = true;

        self
    }

    fn set_border_color(self, state: &mut State, value: Color) -> Self {
        state.style.border_color.insert(self, value);


        state.needs_redraw = true;

        self
    }

    // Border Radius
    fn set_border_radius(self, state: &mut State, value: Length) -> Self {
        state.style.border_radius_top_left.insert(self, value);
        state.style.border_radius_top_right.insert(self, value);
        state.style.border_radius_bottom_left.insert(self, value);
        state.style.border_radius_bottom_right.insert(self, value);

        state.needs_redraw = true;

        self
    }

    fn set_border_radius_top_left(self, state: &mut State, value: Length) -> Self {
        state.style.border_radius_top_left.insert(self, value);

        state.needs_redraw = true;

        self
    }

    fn set_border_radius_top_right(self, state: &mut State, value: Length) -> Self {
        state.style.border_radius_top_right.insert(self, value);

        state.needs_redraw = true;

        self
    }

    fn set_border_radius_bottom_left(self, state: &mut State, value: Length) -> Self {
        state.style.border_radius_bottom_left.insert(self, value);

        state.needs_redraw = true;

        self
    }

    fn set_border_radius_bottom_right(self, state: &mut State, value: Length) -> Self {
        state.style.border_radius_bottom_right.insert(self, value);

        state.needs_redraw = true;

        self
    }

    // Margin
    fn set_margin(self, state: &mut State, value: Length) -> Self {
        state.style.margin_left.insert(self, value);
        state.style.margin_right.insert(self, value);
        state.style.margin_top.insert(self, value);
        state.style.margin_bottom.insert(self, value);

        state.needs_relayout = true;
        state.needs_redraw = true;

        self
    }

    fn set_margin_left(self, state: &mut State, value: Length) -> Self {
        state.style.margin_left.insert(self, value);

        state.needs_relayout = true;
        state.needs_redraw = true;

        self
    }
    fn set_margin_right(self, state: &mut State, value: Length) -> Self {
        state.style.margin_right.insert(self, value);

        state.needs_relayout = true;
        state.needs_redraw = true;

        self
    }
    fn set_margin_top(self, state: &mut State, value: Length) -> Self {
        state.style.margin_top.insert(self, value);

        state.needs_relayout = true;
        state.needs_redraw = true;

        self
    }
    fn set_margin_bottom(self, state: &mut State, value: Length) -> Self {
        state.style.margin_bottom.insert(self, value);

        state.needs_relayout = true;
        state.needs_redraw = true;

        self
    }

    // Padding
    fn set_padding(self, state: &mut State, value: Length) -> Self {
        state.style.padding_left.insert(self, value);
        state.style.padding_right.insert(self, value);
        state.style.padding_top.insert(self, value);
        state.style.padding_bottom.insert(self, value);

        state.needs_relayout = true;
        state.needs_redraw = true;

        self
    }

    fn set_padding_left(self, state: &mut State, value: Length) -> Self {
        state.style.padding_left.insert(self, value);

        state.needs_relayout = true;
        state.needs_redraw = true;

        self
    }
    fn set_padding_right(self, state: &mut State, value: Length) -> Self {
        state.style.padding_right.insert(self, value);

        state.needs_relayout = true;
        state.needs_redraw = true;

        self
    }
    fn set_padding_top(self, state: &mut State, value: Length) -> Self {
        state.style.padding_top.insert(self, value);

        state.needs_relayout = true;
        state.needs_redraw = true;

        self
    }
    fn set_padding_bottom(self, state: &mut State, value: Length) -> Self {
        state.style.padding_bottom.insert(self, value);

        state.needs_relayout = true;
        state.needs_redraw = true;

        self
    }

    // Clipping
    fn set_clip_widget(self, state: &mut State, value: Entity) -> Self {
        state.style.clip_widget.insert(self, value);

        state.needs_relayout = true;
        state.needs_redraw = true;

        self
    }

    fn set_z_order(self, state: &mut State, value: i32) -> Self {
        state.style.z_order.insert(self, value);

        state.needs_relayout = true;
        state.needs_redraw = true;

        self
    }

    fn set_next_focus(self, state: &mut State, value: Entity) -> Self {
        if let Some(data) = state.style.focus_order.get_mut(self) {
            data.next = value;
        } else {
            state.style.focus_order.insert(
                self,
                FocusOrder {
                    next: value,
                    ..Default::default()
                },
            );
        }

        state.needs_relayout = true;
        state.needs_redraw = true;

        self
    }

    fn set_prev_focus(self, state: &mut State, value: Entity) -> Self {
        if let Some(data) = state.style.focus_order.get_mut(self) {
            data.prev = value;
        } else {
            state.style.focus_order.insert(
                self,
                FocusOrder {
                    prev: value,
                    ..Default::default()
                },
            );
        }

        state.needs_relayout = true;
        state.needs_redraw = true;

        self
    }

    fn set_focus_order(self, state: &mut State, next: Entity, prev: Entity) -> Self {
        if let Some(data) = state.style.focus_order.get_mut(self) {
            data.next = next;
            data.prev = prev;
        } else {
            state
                .style
                .focus_order
                .insert(self, FocusOrder { next, prev });
        }

        state.needs_relayout = true;
        state.needs_redraw = true;

        self
    }
}

pub trait PropGet {


    fn is_enabled(self, state: &mut State) -> bool;
    fn is_disabled(self, state: &mut State) -> bool;
    fn is_checked(self, state: &mut State) -> bool;
    fn is_over(self, state: &mut State) -> bool;
    fn is_active(self, state: &mut State) -> bool;
    fn is_focused(self, state: &mut State) -> bool;

    // Display
    fn get_display(&self, state: &mut State) -> Display;

    // Position
    fn get_position(&self, state: &mut State) -> Position;
    fn get_left(&self, state: &mut State) -> Length;
    fn get_right(&self, state: &mut State) -> Length;
    fn get_top(&self, state: &mut State) -> Length;
    fn get_bottom(&self, state: &mut State) -> Length;

    // Size
    fn get_width(&self, state: &mut State) -> Length;
    fn get_height(&self, state: &mut State) -> Length;

    // Size Constraints
    fn get_min_width(&self, state: &mut State) -> Length;
    fn get_max_width(&self, state: &mut State) -> Length;
    fn get_min_height(&self, state: &mut State) -> Length;
    fn get_max_height(&self, state: &mut State) -> Length;

    // Margins
    fn get_margin_left(&self, state: &mut State) -> Length;
    fn get_margin_right(&self, state: &mut State) -> Length;
    fn get_margin_top(&self, state: &mut State) -> Length;
    fn get_margin_bottom(&self, state: &mut State) -> Length;

    // Padding
    fn get_padding_left(&self, state: &mut State) -> Length;
    fn get_padding_right(&self, state: &mut State) -> Length;
    fn get_padding_top(&self, state: &mut State) -> Length;
    fn get_padding_bottom(&self, state: &mut State) -> Length;

    // Border
    fn get_border_width(&self, state: &mut State) -> Length;

    // Flex Container
    fn get_flex_direction(&self, state: &mut State) -> FlexDirection;
    fn get_flex_basis(&self, state: &mut State) -> Length;
    fn get_justify_content(&self, state: &mut State) -> JustifyContent;
    fn get_align_items(&self, state: &mut State) -> AlignItems;

    // Flex Item
    fn get_flex_grow(&self, state: &mut State) -> f32;
    fn get_flex_shrink(&self, state: &mut State) -> f32;
    fn get_align_self(&self, state: &mut State) -> AlignSelf;

    // Tooltip
    fn get_tooltip(&self, state: &mut State) -> String;

    // Text
    fn get_text(&self, state: &mut State) -> String;
}

impl PropGet for Entity {

    fn is_enabled(self, state: &mut State) -> bool {
        if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(self) {
            pseudo_classes.get_enabled()
        } else {
            false
        }
    }
    fn is_disabled(self, state: &mut State) -> bool {
        if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(self) {
            pseudo_classes.get_disabled()
        } else {
            false
        }
    }
    fn is_checked(self, state: &mut State) -> bool {
        if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(self) {
            pseudo_classes.get_checked()
        } else {
            false
        }
    }
    fn is_over(self, state: &mut State) -> bool {
        if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(self) {
            pseudo_classes.get_over()
        } else {
            false
        }
    }
    fn is_active(self, state: &mut State) -> bool {
        if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(self) {
            pseudo_classes.get_active()
        } else {
            false
        }
    }
    fn is_focused(self, state: &mut State) -> bool {
        if let Some(pseudo_classes) = state.style.pseudo_classes.get_mut(self) {
            pseudo_classes.get_focus()
        } else {
            false
        }
    }

    
    // Display
    fn get_display(&self, state: &mut State) -> Display {
        state.style.display.get(*self).cloned().unwrap_or_default()
    }

    // Position
    fn get_position(&self, state: &mut State) -> Position {
        state.style.position.get(*self).cloned().unwrap_or_default()
    }
    fn get_left(&self, state: &mut State) -> Length {
        state.style.left.get(*self).cloned().unwrap_or_default()
    }
    fn get_right(&self, state: &mut State) -> Length {
        state.style.right.get(*self).cloned().unwrap_or_default()
    }
    fn get_top(&self, state: &mut State) -> Length {
        state.style.top.get(*self).cloned().unwrap_or_default()
    }
    fn get_bottom(&self, state: &mut State) -> Length {
        state.style.bottom.get(*self).cloned().unwrap_or_default()
    }

    // Size
    fn get_width(&self, state: &mut State) -> Length {
        state.style.width.get(*self).cloned().unwrap_or_default()
    }

    fn get_height(&self, state: &mut State) -> Length {
        state.style.height.get(*self).cloned().unwrap_or_default()
    }

    // Size Constraints
    fn get_min_width(&self, state: &mut State) -> Length {
        state
            .style
            .min_width
            .get(*self)
            .cloned()
            .unwrap_or_default()
    }

    fn get_max_width(&self, state: &mut State) -> Length {
        state
            .style
            .max_width
            .get(*self)
            .cloned()
            .unwrap_or_default()
    }

    fn get_min_height(&self, state: &mut State) -> Length {
        state
            .style
            .min_height
            .get(*self)
            .cloned()
            .unwrap_or_default()
    }

    fn get_max_height(&self, state: &mut State) -> Length {
        state
            .style
            .max_height
            .get(*self)
            .cloned()
            .unwrap_or_default()
    }

    // Margins
    fn get_margin_left(&self, state: &mut State) -> Length {
        state
            .style
            .margin_left
            .get(*self)
            .cloned()
            .unwrap_or_default()
    }

    fn get_margin_right(&self, state: &mut State) -> Length {
        state
            .style
            .margin_right
            .get(*self)
            .cloned()
            .unwrap_or_default()
    }

    fn get_margin_top(&self, state: &mut State) -> Length {
        state
            .style
            .margin_top
            .get(*self)
            .cloned()
            .unwrap_or_default()
    }

    fn get_margin_bottom(&self, state: &mut State) -> Length {
        state
            .style
            .margin_bottom
            .get(*self)
            .cloned()
            .unwrap_or_default()
    }

    // Padding
    fn get_padding_left(&self, state: &mut State) -> Length {
        state
            .style
            .padding_left
            .get(*self)
            .cloned()
            .unwrap_or_default()
    }

    fn get_padding_right(&self, state: &mut State) -> Length {
        state
            .style
            .padding_right
            .get(*self)
            .cloned()
            .unwrap_or_default()
    }
    fn get_padding_top(&self, state: &mut State) -> Length {
        state
            .style
            .padding_top
            .get(*self)
            .cloned()
            .unwrap_or_default()
    }
    fn get_padding_bottom(&self, state: &mut State) -> Length {
        state
            .style
            .padding_bottom
            .get(*self)
            .cloned()
            .unwrap_or_default()
    }

    // Border
    fn get_border_width(&self, state: &mut State) -> Length {
        state
            .style
            .border_width
            .get(*self)
            .cloned()
            .unwrap_or_default()
    }

    // Flex Container
    fn get_flex_direction(&self, state: &mut State) -> FlexDirection {
        state
            .style
            .flex_direction
            .get(*self)
            .cloned()
            .unwrap_or_default()
    }

    fn get_flex_basis(&self, state: &mut State) -> Length {
        state
            .style
            .flex_basis
            .get(*self)
            .cloned()
            .unwrap_or_default()
    }

    fn get_justify_content(&self, state: &mut State) -> JustifyContent {
        state
            .style
            .justify_content
            .get(*self)
            .cloned()
            .unwrap_or_default()
    }

    fn get_align_items(&self, state: &mut State) -> AlignItems {
        state
            .style
            .align_items
            .get(*self)
            .cloned()
            .unwrap_or_default()
    }

    // Flex Item
    fn get_flex_grow(&self, state: &mut State) -> f32 {
        state
            .style
            .flex_grow
            .get(*self)
            .cloned()
            .unwrap_or_default()
    }

    fn get_flex_shrink(&self, state: &mut State) -> f32 {
        state
            .style
            .flex_shrink
            .get(*self)
            .cloned()
            .unwrap_or_default()
    }

    fn get_align_self(&self, state: &mut State) -> AlignSelf {
        state
            .style
            .align_self
            .get(*self)
            .cloned()
            .unwrap_or_default()
    }

    // Tooltip
    fn get_tooltip(&self, state: &mut State) -> String {
        state.style.tooltip.get(*self).cloned().unwrap_or_default()
    }

    // Text
    fn get_text(&self, state: &mut State) -> String {
        state.style.text.get(*self).cloned().unwrap_or_default().text
    }
}
