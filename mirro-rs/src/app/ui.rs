use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use super::App;

pub fn draw(rect: &mut Frame<impl Backend>, app: &App) {
    let size = rect.size();
    check_size(&size);

    // Vertical layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(3)].as_ref())
        .split(size);

    let title = draw_title(app);
    rect.render_widget(title, chunks[0]);
}

fn check_size(size: &tui::layout::Rect) {
    if size.width < 52 {
        panic!("Require width >= 52, (got {})", size.width);
    }
    if size.height < 28 {
        panic!("Require height >= 28, (got {})", size.height);
    }
}

fn draw_title<'a>(app: &App) -> Paragraph<'a> {
    let count = app.mirrors.countries.len().to_string();
    Paragraph::new(count)
        .style(Style::default().fg(Color::LightCyan))
        .alignment(Alignment::Center)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .border_type(BorderType::Plain),
        )
}
