use chrono::{DateTime, Utc};
use clap::crate_name;
use tracing::error;
use tui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Span, Spans, Text},
    widgets::{Block, Borders, Cell, Gauge, Paragraph, Row, Table},
    Frame,
};
use unicode_width::UnicodeWidthStr;

use super::{
    config::{AvailableMirrors, Colours, Countries, Info},
    state::Widgets,
    App,
};

pub fn draw(rect: &mut Frame<impl Backend>, app: &mut App) {
    let size = rect.size();
    check_size(&size);
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage(32),
                Constraint::Percentage(60),
                Constraint::Percentage(8),
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
                        Constraint::Percentage(20),
                        Constraint::Percentage(60),
                        Constraint::Percentage(20),
                    ]
                    .as_ref(),
                )
                .split(chunks[0]);
            let input = Paragraph::new(app.country_filter.as_ref()).block(
                Block::default()
                    .borders(Borders::ALL)
                    .title(Spans::from(vec![
                        Span::styled(
                            "f".to_string(),
                            Style::default()
                                .fg(action_key_colour(&app.config.colours))
                                .add_modifier(Modifier::BOLD),
                        ),
                        Span::styled(
                            "ilter".to_string(),
                            Style::default().add_modifier(Modifier::BOLD),
                        ),
                    ])),
            );
            rect.render_widget(input, chunks[1]);
            if let Some(widget) = app.state.focused_widget() {
                if widget == &Widgets::CountryFilter {
                    rect.set_cursor(
                        // Put cursor past the end of the input text
                        chunks[1].x + app.country_filter.width() as u16 + 1,
                        // Move one line down, from the border to the input line
                        chunks[1].y + 1,
                    );
                }
            }
            let help = vec![
                Span::raw("Use "),
                Span::styled("<ctrl+[", Style::default().add_modifier(Modifier::BOLD)),
                Span::styled(
                    "key",
                    Style::default()
                        .fg(action_key_colour(&app.config.colours))
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled("]>", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to call a widget to focus"),
            ];
            let help = Paragraph::new(Text::from(Spans::from(help)));
            rect.render_widget(help, chunks[0]);
            let help = vec![
                Span::raw("Use "),
                Span::styled("<ctrl +", Style::default().add_modifier(Modifier::BOLD)),
                Span::styled(
                    " r ",
                    Style::default()
                        .add_modifier(Modifier::BOLD)
                        .add_modifier(Modifier::ITALIC)
                        .fg(Color::Yellow),
                ),
                Span::styled(">", Style::default().add_modifier(Modifier::BOLD)),
                Span::raw(" to rate and export mirrors"),
            ];
            let help = Paragraph::new(Text::from(Spans::from(help)));
            rect.render_widget(help, chunks[2]);
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
            let title = vec![
                Span::styled(
                    "p",
                    Style::default()
                        .fg(action_key_colour(&app.config.colours))
                        .add_modifier(Modifier::BOLD),
                ),
                Span::styled("rotocols", Style::default().add_modifier(Modifier::BOLD)),
            ];
            let input = Block::default()
                .borders(Borders::ALL)
                .title(title)
                .style(Style::default());
            rect.render_widget(input, chunks[1]);
            {
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .margin(1)
                    .constraints(
                        [
                            Constraint::Percentage(33),
                            Constraint::Percentage(33),
                            Constraint::Percentage(33),
                        ]
                        .as_ref(),
                    )
                    .split(chunks[1]);
                let title = vec![
                    Span::styled(
                        "h",
                        Style::default()
                            .fg(action_key_colour(&app.config.colours))
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled("ttps", Style::default().add_modifier(Modifier::BOLD)),
                ];
                let https = Block::default()
                    .borders(Borders::ALL)
                    .title(title)
                    .title_alignment(tui::layout::Alignment::Center);
                let title = vec![
                    Span::styled(
                        "h",
                        Style::default()
                            .fg(action_key_colour(&app.config.colours))
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled("ttp", Style::default().add_modifier(Modifier::BOLD)),
                ];
                let http = Block::default()
                    .borders(Borders::ALL)
                    .title(title)
                    .title_alignment(tui::layout::Alignment::Center);
                let title = vec![
                    Span::styled(
                        "r",
                        Style::default()
                            .fg(action_key_colour(&app.config.colours))
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled("sync", Style::default().add_modifier(Modifier::BOLD)),
                ];
                let rsync = Block::default()
                    .borders(Borders::ALL)
                    .title(title)
                    .title_alignment(tui::layout::Alignment::Center);
                rect.render_widget(https, chunks[0]);
                rect.render_widget(http, chunks[1]);
                rect.render_widget(rsync, chunks[2]);
            }
        }
        {
            let header_cells = ["mirro-rs: 0.1.0"]
                .iter()
                .map(|h| Cell::from(*h).style(Style::default()));
            let header = Row::new(header_cells).height(1).style(Style::default().fg(
                match &app.config.colours {
                    Some(colors) => match &colors.info {
                        Some(available) => app_name(available),
                        None => Color::White,
                    },
                    None => Color::White,
                },
            ));
            let mut count = 0;
            app.mirrors.countries.iter().for_each(|f| {
                count += f.mirrors.len();
            });
            let os = get_os_name();
            let datetime = DateTime::parse_from_rfc3339("2022-02-13T12:08:04.349Z").unwrap();
            let datetime_utc = datetime.with_timezone(&Utc);

            let rows = vec![
                Row::new(vec![
                    (match &app.config.icons {
                        Some(icons) => match &icons.os {
                            Some(icon) => format!("{} os", icon),
                            None => String::from("os"),
                        },
                        None => String::from("os"),
                    }),
                    os,
                ])
                .style(Style::default().fg(match &app.config.colours {
                    Some(colors) => match &colors.info {
                        Some(available) => os_header(available),
                        None => Color::White,
                    },
                    None => Color::White,
                })),
                Row::new(vec![
                    (match &app.config.icons {
                        Some(icons) => match &icons.countries {
                            Some(icon) => format!("{} countries", icon),
                            None => String::from("countries"),
                        },
                        None => String::from("countries"),
                    }),
                    app.mirrors.countries.len().to_string(),
                ])
                .style(Style::default().fg(match &app.config.colours {
                    Some(colors) => match &colors.info {
                        Some(available) => countries_header(available),
                        None => Color::White,
                    },
                    None => Color::White,
                })),
                Row::new(vec![
                    (match &app.config.icons {
                        Some(icons) => match &icons.mirrors {
                            Some(icon) => format!("{} mirrors", icon),
                            None => String::from("mirrors"),
                        },
                        None => String::from("mirrors"),
                    }),
                    count.to_string(),
                ])
                .style(Style::default().fg(match &app.config.colours {
                    Some(colors) => match &colors.info {
                        Some(available) => mirrors_header(available),
                        None => Color::White,
                    },
                    None => Color::White,
                })),
                Row::new(vec![
                    (match &app.config.icons {
                        Some(icons) => match &icons.last_checked {
                            Some(icon) => format!("{} last checked", icon),
                            None => String::from("last checked"),
                        },
                        None => String::from("last checked"),
                    }),
                    datetime_utc.format("%d %h %H:%M").to_string(),
                ])
                .style(Style::default().fg(match &app.config.colours {
                    Some(colors) => match &colors.info {
                        Some(available) => last_checked_header(available),
                        None => Color::White,
                    },
                    None => Color::White,
                })),
                Row::new(vec![
                    (match &app.config.icons {
                        Some(icons) => match &icons.now {
                            Some(icon) => format!("{} now", icon),
                            None => String::from("now"),
                        },
                        None => String::from("now"),
                    }),
                    app.clock.format("%d %h %H:%M").to_string(),
                ])
                .style(Style::default().fg(match &app.config.colours {
                    Some(colors) => match &colors.info {
                        Some(available) => now_header(available),
                        None => Color::White,
                    },
                    None => Color::White,
                })),
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
                    .fg(match &app.config.colours {
                        Some(colors) => match &colors.available_mirrors {
                            Some(available) => heading_colour(available),
                            None => Color::White,
                        },
                        None => Color::White,
                    })
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
        let (fg, bg, reversed): (String, String, bool) = match &app.config.colours {
            Some(cols) => match &cols.available_mirrors {
                Some(cols) => match &cols.highlight_fg {
                    Some(fg) => match &cols.highlight_bg {
                        Some(bg) => match &cols.reverse {
                            Some(reversed) => (fg.to_string(), bg.to_string(), *reversed),
                            None => (fg.to_string(), bg.to_string(), false),
                        },
                        None => match &cols.reverse {
                            Some(vals) => (fg.to_string(), "d3d3d3".to_owned(), *vals),
                            None => (fg.to_string(), "d3d3d3".to_owned(), false),
                        },
                    },
                    None => {
                        let fg = "d3d3d3";
                        match &cols.highlight_bg {
                            Some(bg) => match &cols.reverse {
                                Some(reversed) => (fg.to_string(), bg.to_string(), *reversed),
                                None => (fg.to_string(), bg.to_string(), false),
                            },
                            None => match &cols.reverse {
                                Some(vals) => (fg.to_string(), "d3d3d3".to_owned(), *vals),
                                None => (fg.to_string(), "d3d3d3".to_owned(), false),
                            },
                        }
                    }
                },
                None => ("d3d3d3".to_owned(), "d3d3d3".to_owned(), false),
            },
            None => ("d3d3d3".to_owned(), "d3d3d3".to_owned(), false),
        };
        let bg = rgb_from_hex(bg);
        let fg = rgb_from_hex(fg);
        let selected_style = Style::default()
            .fg(match fg {
                Some(colors) => Color::Rgb(colors.0, colors.1, colors.2),
                None => Color::Gray,
            })
            .bg(match bg {
                Some(colors) => Color::Rgb(colors.0, colors.1, colors.2),
                None => Color::Gray,
            })
            .add_modifier(Modifier::BOLD);
        let symbol = match &app.config.icons {
            Some(val) => match val.highlight_symbol_mirrors {
                Some(char) => char.to_string(),
                None => String::from(" "),
            },
            None => String::from(" "),
        };
        let style = match reversed {
            true => Style::default()
                .fg(match &app.config.colours {
                    Some(colors) => match &colors.available_mirrors {
                        Some(available) => border_colour_mirrors(available),
                        None => Color::White,
                    },
                    None => Color::White,
                })
                .add_modifier(Modifier::REVERSED),
            false => Style::default().fg(match &app.config.colours {
                Some(colors) => match &colors.available_mirrors {
                    Some(available) => border_colour_mirrors(available),
                    None => Color::White,
                },
                None => Color::White,
            }),
        };

        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(60), Constraint::Percentage(40)].as_ref())
            .split(chunks[1]);
        let title = vec![
            Span::styled(
                "a",
                Style::default()
                    .fg(action_key_colour(&app.config.colours))
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                "vailable mirrors",
                Style::default().add_modifier(Modifier::BOLD),
            ),
        ];
        let t = Table::new(rows)
            .header(header)
            .block(
                Block::default()
                    .title(title)
                    .borders(Borders::ALL)
                    .border_style(style),
            )
            .highlight_style(selected_style)
            .highlight_symbol(&symbol)
            .widths(&[
                Constraint::Percentage(50),
                Constraint::Length(30),
                Constraint::Min(10),
            ]);
        rect.render_stateful_widget(t, chunks[0], &mut app.table);

        {
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Percentage(60), Constraint::Percentage(40)].as_ref())
                .split(chunks[1]);
            let rows = app.selected_countries.iter().map(|resp| {
                let mut item_name = resp.country.country.as_str();
                if item_name.is_empty() {
                    item_name = "misc"
                }
                let row = vec![item_name.to_owned()];
                Row::new(row.into_iter())
            });

            let (fg, bg, reversed): (String, String, bool) = match &app.config.colours {
                Some(cols) => match &cols.countries {
                    Some(cols) => match &cols.highlight_fg {
                        Some(fg) => match &cols.highlight_bg {
                            Some(bg) => match &cols.reverse {
                                Some(reversed) => (fg.to_string(), bg.to_string(), *reversed),
                                None => (fg.to_string(), bg.to_string(), false),
                            },
                            None => match &cols.reverse {
                                Some(vals) => (fg.to_string(), "d3d3d3".to_owned(), *vals),
                                None => (fg.to_string(), "d3d3d3".to_owned(), false),
                            },
                        },
                        None => {
                            let fg = "d3d3d3";
                            match &cols.highlight_bg {
                                Some(bg) => match &cols.reverse {
                                    Some(reversed) => (fg.to_string(), bg.to_string(), *reversed),
                                    None => (fg.to_string(), bg.to_string(), false),
                                },
                                None => match &cols.reverse {
                                    Some(vals) => (fg.to_string(), "d3d3d3".to_owned(), *vals),
                                    None => (fg.to_string(), "d3d3d3".to_owned(), false),
                                },
                            }
                        }
                    },
                    None => ("d3d3d3".to_owned(), "d3d3d3".to_owned(), false),
                },
                None => ("d3d3d3".to_owned(), "d3d3d3".to_owned(), false),
            };
            let bg = rgb_from_hex(bg);
            let fg = rgb_from_hex(fg);
            let selected_style = Style::default()
                .fg(match fg {
                    Some(colors) => Color::Rgb(colors.0, colors.1, colors.2),
                    None => Color::Gray,
                })
                .bg(match bg {
                    Some(colors) => Color::Rgb(colors.0, colors.1, colors.2),
                    None => Color::Gray,
                })
                .add_modifier(Modifier::BOLD);
            let symbol = match &app.config.icons {
                Some(val) => match val.highlight_symbol_countries {
                    Some(char) => char.to_string(),
                    None => String::from(" "),
                },
                None => String::from(" "),
            };
            let style = match reversed {
                true => Style::default()
                    .fg(match &app.config.colours {
                        Some(colors) => match &colors.countries {
                            Some(available) => border_colour_countries(available),
                            None => Color::White,
                        },
                        None => Color::White,
                    })
                    .add_modifier(Modifier::REVERSED),
                false => Style::default().fg(match &app.config.colours {
                    Some(colors) => match &colors.countries {
                        Some(available) => border_colour_countries(available),
                        None => Color::White,
                    },
                    None => Color::White,
                }),
            };
            let header_cells = ["marked for saving:"].iter().map(|h| {
                Cell::from(*h).style(
                    Style::default()
                        .fg(match &app.config.colours {
                            Some(colors) => match &colors.countries {
                                Some(available) => heading_colour_countries(available),
                                None => Color::White,
                            },
                            None => Color::White,
                        })
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
                                Style::default()
                                    .fg(action_key_colour(&app.config.colours))
                                    .add_modifier(Modifier::BOLD),
                            ),
                            Span::styled(
                                "untries".to_string(),
                                Style::default().add_modifier(Modifier::BOLD),
                            ),
                        ]))
                        .borders(Borders::ALL)
                        .border_style(style),
                )
                .highlight_symbol(&symbol)
                .highlight_style(selected_style)
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
                        .fg(match &app.config.colours {
                            Some(colors) => match &colors.countries {
                                Some(available) => heading_colour_countries(available),
                                None => Color::Blue,
                            },
                            None => Color::Blue,
                        })
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
                    Constraint::Percentage(100),
                    Constraint::Length(30),
                    Constraint::Min(10),
                ]);
            rect.render_widget(t, chunks[1]);
        }
    }
    let gauge = Gauge::default()
        .block(
            Block::default()
                .borders(if app.selected_countries.is_empty() {
                    Borders::NONE
                } else {
                    Borders::ALL
                })
                .title(if app.selected_countries.is_empty() {
                    ""
                } else {
                    "progress"
                }),
        )
        .gauge_style(if app.selected_countries.is_empty() {
            Style::default()
        } else {
            Style::default().fg(Color::Cyan)
        })
        .percent(70);
    rect.render_widget(gauge, chunks[2]);
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

fn action_key_colour(colours: &Option<Colours>) -> tui::style::Color {
    match &colours {
        Some(val) => match &val.action_key {
            Some(color) => {
                if let Some((red, green, blue)) = rgb_from_hex(color.to_string()) {
                    Color::Rgb(red, green, blue)
                } else {
                    Color::White
                }
            }
            None => Color::White,
        },
        None => Color::White,
    }
}
fn rgb_from_hex(val: String) -> Option<(u8, u8, u8)> {
    if val.chars().into_iter().count() == 6 {
        match u8::from_str_radix(&val[0..2], 16) {
            Ok(red) => match u8::from_str_radix(&val[2..4], 16) {
                Ok(green) => match u8::from_str_radix(&val[4..6], 16) {
                    Ok(blue) => Some((red, green, blue)),
                    Err(e) => {
                        error!("{e}");
                        None
                    }
                },
                Err(e) => {
                    error!("{e}");
                    None
                }
            },
            Err(e) => {
                error!("{e}");
                None
            }
        }
    } else {
        Some((255, 255, 255))
    }
}

fn heading_colour(colours: &AvailableMirrors) -> tui::style::Color {
    match &colours.heading {
        Some(col) => {
            if let Some((red, green, blue)) = rgb_from_hex(col.to_string()) {
                Color::Rgb(red, green, blue)
            } else {
                Color::Blue
            }
        }
        None => Color::Blue,
    }
}
fn border_colour_mirrors(colours: &AvailableMirrors) -> tui::style::Color {
    match &colours.border {
        Some(col) => {
            if let Some((red, green, blue)) = rgb_from_hex(col.to_string()) {
                Color::Rgb(red, green, blue)
            } else {
                Color::Blue
            }
        }
        None => Color::Blue,
    }
}

fn border_colour_countries(colours: &Countries) -> tui::style::Color {
    match &colours.border {
        Some(col) => {
            if let Some((red, green, blue)) = rgb_from_hex(col.to_string()) {
                Color::Rgb(red, green, blue)
            } else {
                Color::Blue
            }
        }
        None => Color::Blue,
    }
}
fn heading_colour_countries(colours: &Countries) -> tui::style::Color {
    match &colours.heading {
        Some(col) => {
            if let Some((red, green, blue)) = rgb_from_hex(col.to_string()) {
                Color::Rgb(red, green, blue)
            } else {
                Color::Blue
            }
        }
        None => Color::Blue,
    }
}

fn os_header(colours: &Info) -> tui::style::Color {
    match &colours.os {
        Some(col) => {
            if let Some((red, green, blue)) = rgb_from_hex(col.to_string()) {
                Color::Rgb(red, green, blue)
            } else {
                Color::Blue
            }
        }
        None => Color::Blue,
    }
}
fn countries_header(colours: &Info) -> tui::style::Color {
    match &colours.countries {
        Some(col) => {
            if let Some((red, green, blue)) = rgb_from_hex(col.to_string()) {
                Color::Rgb(red, green, blue)
            } else {
                Color::Blue
            }
        }
        None => Color::Blue,
    }
}
fn mirrors_header(colours: &Info) -> tui::style::Color {
    match &colours.mirrors {
        Some(col) => {
            if let Some((red, green, blue)) = rgb_from_hex(col.to_string()) {
                Color::Rgb(red, green, blue)
            } else {
                Color::Blue
            }
        }
        None => Color::Blue,
    }
}

fn last_checked_header(colours: &Info) -> tui::style::Color {
    match &colours.last_checked {
        Some(col) => {
            if let Some((red, green, blue)) = rgb_from_hex(col.to_string()) {
                Color::Rgb(red, green, blue)
            } else {
                Color::Blue
            }
        }
        None => Color::Blue,
    }
}

fn now_header(colours: &Info) -> tui::style::Color {
    match &colours.now {
        Some(col) => {
            if let Some((red, green, blue)) = rgb_from_hex(col.to_string()) {
                Color::Rgb(red, green, blue)
            } else {
                Color::Blue
            }
        }
        None => Color::Blue,
    }
}

fn app_name(colours: &Info) -> tui::style::Color {
    match &colours.app {
        Some(col) => {
            if let Some((red, green, blue)) = rgb_from_hex(col.to_string()) {
                Color::Rgb(red, green, blue)
            } else {
                Color::Blue
            }
        }
        None => Color::Blue,
    }
}
