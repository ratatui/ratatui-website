use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    widgets::{Block, Paragraph, Widget},
};

#[allow(dead_code)]
// ANCHOR: grid
struct Grid {
    cols: usize,
    rows: usize,
}
// ANCHOR_END: grid

// ANCHOR: widget
impl Widget for Grid {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let col_constraints = (0..self.cols).map(|_| Constraint::Length(9));
        let row_constraints = (0..self.rows).map(|_| Constraint::Length(3));
        let horizontal = Layout::horizontal(col_constraints).spacing(1);
        let vertical = Layout::vertical(row_constraints).spacing(1);

        let rows = vertical.split(area);
        let cells = rows.iter().flat_map(|&row| horizontal.split(row).to_vec());

        for (i, cell) in cells.enumerate() {
            Paragraph::new(format!("Area {:02}", i + 1))
                .block(Block::bordered())
                .render(cell, buf);
        }
    }
}
// ANCHOR_END: widget

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn render_grid() {
        let mut buf = Buffer::empty(Rect::new(0, 0, 39, 15));
        let grid = Grid { cols: 4, rows: 4 };
        grid.render(buf.area, &mut buf);
        assert_eq!(
            buf,
            Buffer::with_lines([
                "┌───────┐ ┌───────┐ ┌───────┐ ┌───────┐",
                "│Area 01│ │Area 02│ │Area 03│ │Area 04│",
                "└───────┘ └───────┘ └───────┘ └───────┘",
                "                                       ",
                "┌───────┐ ┌───────┐ ┌───────┐ ┌───────┐",
                "│Area 05│ │Area 06│ │Area 07│ │Area 08│",
                "└───────┘ └───────┘ └───────┘ └───────┘",
                "                                       ",
                "┌───────┐ ┌───────┐ ┌───────┐ ┌───────┐",
                "│Area 09│ │Area 10│ │Area 11│ │Area 12│",
                "└───────┘ └───────┘ └───────┘ └───────┘",
                "                                       ",
                "┌───────┐ ┌───────┐ ┌───────┐ ┌───────┐",
                "│Area 13│ │Area 14│ │Area 15│ │Area 16│",
                "└───────┘ └───────┘ └───────┘ └───────┘",
            ])
        )
    }
}
