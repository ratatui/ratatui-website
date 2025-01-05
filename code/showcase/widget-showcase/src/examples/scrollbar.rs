use lipsum::lipsum;
use ratatui::{
    layout::{Margin, Rect},
    style::Stylize,
    text::Text,
    widgets::{Block, Borders, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, Wrap},
    Frame,
};

pub fn render(frame: &mut Frame) {
    let word_count = frame.area().width * frame.area().height / 3;
    let text = lipsum(word_count as usize);

    let Rect {
        x,
        y,
        width,
        height,
    } = frame.area();
    let main_area = Rect::new(x, y, width - 4, height - 4);

    // wrap (using textwrap) a bit wider than the frame to make sure we have a scrollbar
    let wrapped = textwrap::wrap(&text, (width as f64 * 1.5) as usize);
    let text = Text::from_iter(wrapped).dim();

    let block = Block::bordered().title("Scrollbar").borders(Borders::ALL);

    let text_height = text.height();
    let text_width = text.width();
    let vertical_scroll = text_height as f64 / 3.0;
    let horizontal_scroll = text_width as f64 / 3.0;

    let para = Paragraph::new(text)
        .block(block)
        .scroll((vertical_scroll as u16, horizontal_scroll as u16));
    frame.render_widget(para, main_area);

    let scrollbar = Scrollbar::new(ScrollbarOrientation::HorizontalBottom);
    let mut scrollbar_state = ScrollbarState::new(text_width).position(horizontal_scroll as usize);
    let area = main_area.inner(Margin::new(1, 0));
    frame.render_stateful_widget(scrollbar, area, &mut scrollbar_state);

    let scrollbar = Scrollbar::new(ScrollbarOrientation::HorizontalBottom);
    let mut scrollbar_state = ScrollbarState::new(text_width).position(0 as usize);
    let area = Rect::new(x + 1, y, width - 6, height - 2);
    frame.render_stateful_widget(scrollbar, area, &mut scrollbar_state);

    let scrollbar = Scrollbar::new(ScrollbarOrientation::HorizontalBottom);
    let mut scrollbar_state = ScrollbarState::new(text_width).position(text_width as usize);
    let area = Rect::new(x + 1, y, width - 6, height);
    frame.render_stateful_widget(scrollbar, area, &mut scrollbar_state);

    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);
    let mut scrollbar_state = ScrollbarState::new(text_height).position(vertical_scroll as usize);
    let area = main_area.inner(Margin::new(0, 1));
    frame.render_stateful_widget(scrollbar, area, &mut scrollbar_state);

    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);
    let mut scrollbar_state = ScrollbarState::new(text_height).position(0);
    let area = Rect::new(x, y + 1, width - 2, height - 6);
    frame.render_stateful_widget(scrollbar, area, &mut scrollbar_state);

    let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight);
    let mut scrollbar_state = ScrollbarState::new(text_height).position(text_height as usize);
    let area = Rect::new(x, y + 1, width, height - 6);
    frame.render_stateful_widget(scrollbar, area, &mut scrollbar_state);
}
