use ropey::Rope;
use tower_lsp::lsp_types::{Position, Range};
use tree_sitter::{InputEdit, Node, Point};

pub trait NodeExt {
    fn range(&self) -> Range;
}

impl<'a> NodeExt for Node<'a> {
    fn range(&self) -> Range {
        let start = self.start_position();
        let end = self.end_position();

        Range {
            start: to_position(start),
            end: to_position(end),
        }
    }
}

pub trait RopeExt {
    fn to_byte(&self, position: Position) -> usize;
    fn to_position(&self, offset: usize) -> Position;
    fn to_input_edit(&self, range: Range, text: &str) -> InputEdit;
}

impl RopeExt for Rope {
    fn to_byte(&self, position: Position) -> usize {
        let start_line = self.line_to_byte(position.line as usize);
        start_line + position.character as usize
    }

    fn to_position(&self, mut offset: usize) -> Position {
        offset = offset.min(self.len_bytes());
        let mut low = 0usize;
        let mut high = self.len_lines();
        if high == 0 {
            return Position {
                line: 0,
                character: offset as u32,
            };
        }
        while low < high {
            let mid = low + (high - low) / 2;
            if self.line_to_byte(mid) > offset {
                high = mid;
            } else {
                low = mid + 1;
            }
        }
        let line = low - 1;
        let character = offset - self.line_to_byte(line);
        Position::new(line as u32, character as u32)
    }

    fn to_input_edit(&self, range: Range, text: &str) -> InputEdit {
        let start = range.start;
        let end = range.end;

        let start_byte = self.to_byte(start);
        let start_position = to_point(start);

        let new_end_byte = start_byte + text.len();
        let new_end_position = self.to_position(new_end_byte);
        let new_end_position = to_point(new_end_position);

        let old_end_byte = self.to_byte(end);
        let old_end_position = to_point(end);

        InputEdit {
            start_byte,
            old_end_byte,
            new_end_byte,
            start_position,
            old_end_position,
            new_end_position,
        }
    }
}

pub fn intersects(a: (Point, Point), b: (Point, Point)) -> bool {
    let (a_start, a_end) = a;
    let (b_start, b_end) = b;
    a_start.row <= b_end.row
        && a_end.row >= b_start.row
        && a_start.column <= b_end.column
        && a_end.column >= b_start.column
}

pub fn to_point(pos: Position) -> Point {
    Point::new(pos.line as usize, pos.character as usize)
}
pub fn to_position(point: Point) -> Position {
    Position {
        line: point.row as u32,
        character: point.column as u32,
    }
}
