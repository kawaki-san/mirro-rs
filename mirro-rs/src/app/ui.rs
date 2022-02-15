use chrono::{DateTime, Utc};
use clap::crate_name;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Cell, Paragraph, Row, Table},
    Frame,
};

use super::App;

pub fn draw(rect: &mut Frame<impl Backend>, app: &mut App) {
    let size = rect.size();
    check_size(&size);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(20),
                Constraint::Percentage(70),
                Constraint::Percentage(10),
            ]
            .as_ref(),
        )
        .split(rect.size());
    let block_0 = Block::default()
        .borders(Borders::ALL)
        .title(Spans::from(section_title(format!(
            "{} - {}",
            crate_name!(),
            match app.state.focused_widget() {
                Some(w) => w.to_string(),
                None => String::default(),
            }
        ))));
    {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(35),
                    Constraint::Percentage(30),
                    Constraint::Percentage(35),
                ]
                .as_ref(),
            )
            .split(chunks[0]);
        {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(25),
                        Constraint::Percentage(50),
                        Constraint::Percentage(25),
                    ]
                    .as_ref(),
                )
                .split(chunks[0]);
            let input = Paragraph::new(app.country_filter.as_ref())
                .style(if app.country_filter.is_empty() {
                    Style::default()
                } else {
                    Style::default().fg(Color::Green)
                })
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title(Spans::from(vec![
                            Span::styled(
                                "f".to_string(),
                                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                            ),
                            Span::styled(
                                "ilter".to_string(),
                                Style::default().add_modifier(Modifier::BOLD),
                            ),
                        ])),
                );
            rect.render_widget(input, chunks[1]);
            let help = Paragraph::new("Press").style(Style::default());
            rect.render_widget(help, chunks[0]);
        }
        {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(1)
                .constraints(
                    [
                        Constraint::Percentage(5),
                        Constraint::Percentage(90),
                        Constraint::Percentage(5),
                    ]
                    .as_ref(),
                )
                .split(chunks[1]);
            let input = Block::default()
                .borders(Borders::ALL)
                .title(widget_title("protocols"))
                .style(Style::default());
            rect.render_widget(input, chunks[1]);
        }
        {
            let header_cells = ["mirro-rs: 0.1.0"]
                .iter()
                .map(|h| Cell::from(*h).style(Style::default()));
            let header = Row::new(header_cells)
                .height(1)
                .style(Style::default().fg(Color::Red));
            let mut count = 0;
            app.mirrors.countries.iter().for_each(|f| {
                count += f.mirrors.len();
            });
            let os = get_os_name();
            let datetime = DateTime::parse_from_rfc3339("2022-02-13T12:08:04.349Z").unwrap();
            let datetime_utc = datetime.with_timezone(&Utc);

            let rows = vec![
                Row::new(vec![table_field("os"), os]).style(Style::default().fg(Color::Blue)),
                Row::new(vec![
                    table_field("countries"),
                    app.mirrors.countries.len().to_string(),
                ])
                .style(Style::default().fg(Color::Yellow)),
                Row::new(vec![table_field("mirrors"), count.to_string()])
                    .style(Style::default().fg(Color::Magenta)),
                Row::new(vec![
                    table_field("last checked"),
                    datetime_utc.format("%d %h %H:%M").to_string(),
                ])
                .style(Style::default()),
                Row::new(vec![
                    table_field("now"),
                    app.clock.format("%d %h %H:%M:%S").to_string(),
                ])
                .style(Style::default().fg(Color::Cyan)),
            ];
            let t = Table::new(rows)
                .header(header)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .border_style(Style::default()),
                )
                .highlight_symbol(" ")
                .widths(&[
                    Constraint::Percentage(40),
                    Constraint::Length(30),
                    Constraint::Min(10),
                ]);
            rect.render_widget(t, chunks[2]);
        }
    }
    {
        let header_cells = ["Country:", "Mirrors:"].iter().map(|h| {
            Cell::from(*h).style(
                Style::default()
                    .fg(Color::White)
                    .add_modifier(Modifier::BOLD),
            )
        });
        let header = Row::new(header_cells).height(1);
        let rows = app.mirrors.countries.iter().filter_map(|resp| {
            if resp
                .country
                .to_lowercase()
                .contains(&app.country_filter.to_lowercase())
            {
                let mut item_name = resp.country.as_str();
                if item_name.is_empty() {
                    item_name = "misc"
                }
                let row = vec![item_name.to_owned(), resp.mirrors.len().to_string()];
                return Some(Row::new(row.into_iter()));
            }
            None
        });
        let selected_style = Style::default()
            .fg(Color::Blue)
            .add_modifier(Modifier::BOLD)
            .bg(Color::DarkGray)
            .add_modifier(Modifier::REVERSED);
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(65), Constraint::Percentage(35)].as_ref())
            .split(chunks[1]);
        let t = Table::new(rows)
            .header(header)
            .block(
                Block::default()
                    .title(widget_title("available mirrors"))
                    .borders(Borders::ALL)
                    .border_style(Style::default()),
            )
            .highlight_style(selected_style)
            .highlight_symbol(" ")
            .widths(&[
                Constraint::Percentage(50),
                Constraint::Length(30),
                Constraint::Min(10),
            ]);
        rect.render_stateful_widget(t, chunks[0], &mut app.table);

        {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(65), Constraint::Percentage(35)].as_ref())
                .split(chunks[1]);
            let rows = app.selected_countries.iter().map(|resp| {
                let mut item_name = resp.country.country.as_str();
                if item_name.is_empty() {
                    item_name = "misc"
                }
                let row = vec![item_name.to_owned()];
                Row::new(row.into_iter())
            });

            let header_cells = ["marked for saving:"].iter().map(|h| {
                Cell::from(*h).style(
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                )
            });
            let header = Row::new(header_cells).height(1);
            let t = Table::new(rows)
                .header(header)
                .block(
                    Block::default()
                        .title(Spans::from(vec![
                            Span::styled(
                                "c".to_string(),
                                Style::default().add_modifier(Modifier::BOLD),
                            ),
                            Span::styled(
                                "o".to_string(),
                                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                            ),
                            Span::styled(
                                "untries".to_string(),
                                Style::default().add_modifier(Modifier::BOLD),
                            ),
                        ]))
                        .borders(Borders::ALL)
                        .border_style(Style::default()),
                )
                .highlight_symbol(" ")
                .highlight_style(selected_style.patch(Style::default().fg(Color::Yellow)))
                .widths(&[
                    Constraint::Percentage(80),
                    Constraint::Length(30),
                    Constraint::Min(10),
                ]);
            rect.render_stateful_widget(t, chunks[0], &mut app.selected_table);
            let rows = app.focused_country().mirrors.iter().map(|resp| {
                let mut item_name = resp.url.as_str();
                if item_name.is_empty() {
                    item_name = "misc"
                }
                let row = vec![item_name.to_owned()];
                Row::new(row.into_iter())
            });

            let header_cells = ["per country:"].iter().map(|h| {
                Cell::from(*h).style(
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                )
            });
            let header = Row::new(header_cells).height(1);
            let t = Table::new(rows)
                .header(header)
                .block(
                    Block::default()
                        .title("mirrors")
                        .borders(Borders::ALL)
                        .border_style(Style::default()),
                )
                .widths(&[
                    Constraint::Percentage(80),
                    Constraint::Length(30),
                    Constraint::Min(10),
                ]);
            rect.render_widget(t, chunks[1]);
        }
    }
    rect.render_widget(block_0, chunks[0]);
}

fn get_os_name() -> String {
    let os = std::fs::read_to_string("/etc/os-release").unwrap();
    let os: Vec<_> = os.lines().collect();
    let os = os.get(0).unwrap();
    let os: Vec<_> = os.split('\"').collect();
    os.get(1).unwrap().to_string()
}

fn section_title(title: impl AsRef<str>) -> Vec<Span<'static>> {
    vec![Span::styled(
        format!(" {} ", title.as_ref()),
        Style::default().add_modifier(Modifier::REVERSED),
    )]
}

fn check_size(size: &tui::layout::Rect) {
    if size.width < 52 {
        panic!("Require width >= 52, (got {})", size.width);
    }
    if size.height < 28 {
        panic!("Require height >= 28, (got {})", size.height);
    }
}

fn table_field(text: &str) -> String {
    format!(" {text}")
}

fn widget_title(title: &str) -> Spans {
    Spans::from(vec![
        Span::styled(
            title.get(0..1).unwrap().to_string(),
            Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
        ),
        Span::styled(
            title.get(1..title.len()).unwrap().to_string(),
            Style::default().add_modifier(Modifier::BOLD),
        ),
    ])
}
