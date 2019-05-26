pub mod column; // TODO: Make private
mod row;

pub mod button;
pub mod checkbox;
pub mod panel;
pub mod radio;
pub mod slider;
pub mod text;

pub use button::Button;
pub use checkbox::Checkbox;
pub use column::Column;
pub use panel::Panel;
pub use radio::Radio;
pub use row::Row;
pub use slider::Slider;
pub use text::Text;

use crate::graphics::Point;
use crate::ui::{Event, Hasher, Layout, MouseCursor, Node};

pub trait Widget {
    type Msg;
    type Renderer;

    fn node(&self, renderer: &Self::Renderer) -> Node;

    fn on_event(
        &mut self,
        _event: Event,
        _layout: Layout,
        _cursor_position: Point,
        _messages: &mut Vec<Self::Msg>,
    ) {
    }

    fn draw(
        &self,
        renderer: &mut Self::Renderer,
        layout: Layout,
        cursor_position: Point,
    ) -> MouseCursor;

    fn hash(&self, state: &mut Hasher);
}
