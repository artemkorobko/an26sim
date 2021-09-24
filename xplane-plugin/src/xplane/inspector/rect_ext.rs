use xplm::geometry::Rect;

const PADDING: i32 = 10;
const LINE_HEIGHT: i32 = 12;
const VALUE_OFFSET: i32 = 80;

pub(super) trait RectExt {
    fn from_size(width: i32, height: i32) -> Rect<i32>;
    fn to_left_section(&self) -> Rect<i32>;
    fn to_right_section(&self) -> Rect<i32>;
    fn to_value_line(&self) -> Rect<i32>;
    fn to_next_block(&self) -> Rect<i32>;
    fn to_next_line(&self) -> Rect<i32>;
}

impl RectExt for Rect<i32> {
    fn from_size(width: i32, height: i32) -> Rect<i32> {
        let left = PADDING;
        let top = height + PADDING;
        let right = left + width;
        let bottom = PADDING;
        Rect::from_left_top_right_bottom(left, top, right, bottom)
    }

    fn to_left_section(&self) -> Rect<i32> {
        let left = self.left() + PADDING;
        let top = self.top() - (PADDING * 2);
        let right = self.right() - PADDING;
        let bottom = top - LINE_HEIGHT;
        Rect::from_left_top_right_bottom(left, top, right, bottom)
    }

    fn to_right_section(&self) -> Rect<i32> {
        let left = self.left() + ((self.left() + self.right()) / 2);
        let top = self.top() - (PADDING * 2);
        let right = self.right() - PADDING;
        let bottom = top - LINE_HEIGHT;
        Rect::from_left_top_right_bottom(left, top, right, bottom)
    }

    fn to_value_line(&self) -> Rect<i32> {
        let left = self.left() + VALUE_OFFSET;
        let top = self.top();
        let right = self.right();
        let bottom = self.bottom();
        Rect::from_left_top_right_bottom(left, top, right, bottom)
    }

    fn to_next_block(&self) -> Rect<i32> {
        let left = self.left();
        let top = self.top() - (LINE_HEIGHT * 2);
        let right = self.right();
        let bottom = top - LINE_HEIGHT;
        Rect::from_left_top_right_bottom(left, top, right, bottom)
    }

    fn to_next_line(&self) -> Rect<i32> {
        let left = self.left();
        let top = self.top() - LINE_HEIGHT;
        let right = self.right();
        let bottom = top - LINE_HEIGHT;
        Rect::from_left_top_right_bottom(left, top, right, bottom)
    }
}
