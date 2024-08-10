use ratatui::{prelude::*, widgets::*};

pub fn render(frame: &mut Frame) {
    let text = vec![
        Line::from("Hello, Ratatui".white().on_blue().italic()),
        Line::from("This is a colorful line".red().underlined()),
        Line::from("Be bold with your paragraphs".bold()),
    ];

    let paragraph = Paragraph::new(text)
        .bold()
        .white()
        .alignment(Alignment::Center);

    frame.render_widget(paragraph, frame.area());
}
