use super::Renderer;

use crate::graphics::{Point, Rectangle, Sprite};
use crate::ui::core::{widget::radio, MouseCursor};

pub type Radio<M> = radio::Radio<M, Renderer>;

const SPRITE: Rectangle<u16> = Rectangle {
    x: 98,
    y: 28,
    width: 28,
    height: 28,
};

impl radio::Renderer for Renderer {
    fn draw(
        &mut self,
        is_selected: bool,
        bounds: Rectangle<f32>,
        bounds_with_label: Rectangle<f32>,
        cursor_position: Point,
    ) -> MouseCursor {
        let mouse_over = bounds_with_label.contains(cursor_position);

        self.sprites.add(Sprite {
            source: Rectangle {
                x: SPRITE.x + (if mouse_over { SPRITE.width } else { 0 }),
                ..SPRITE
            },
            position: Point::new(bounds.x, bounds.y),
            scale: (1.0, 1.0),
        });

        if is_selected {
            self.sprites.add(Sprite {
                source: Rectangle {
                    x: SPRITE.x + SPRITE.width * 2,
                    ..SPRITE
                },
                position: Point::new(bounds.x, bounds.y),
                scale: (1.0, 1.0),
            });
        }

        if mouse_over {
            MouseCursor::Pointer
        } else {
            MouseCursor::Default
        }
    }
}
