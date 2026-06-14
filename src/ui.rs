use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
    Frame,
};

use crate::app::{App, ModalState};

pub fn ui(f: &mut Frame, app: &App) {
    let size = f.area();
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)].as_ref())
        .split(size);

    let items: Vec<ListItem> = app
        .items
        .iter()
        .enumerate()
        .map(|(i, (item, selected))| {
            let checkbox = if *selected { "[x]" } else { "[ ]" };
            let content = format!("{} {} - {}", checkbox, item.name, item.description);
            let mut style = Style::default().fg(Color::White);
            if i == app.selected_index {
                style = style.fg(Color::Yellow).add_modifier(Modifier::BOLD);
            }
            if !*selected {
                style = style.fg(Color::DarkGray);
            }
            ListItem::new(content).style(style)
        })
        .collect();

    let items_list = List::new(items)
        .block(Block::default().title("ArchCleaner - Unified Cleaning").borders(Borders::ALL));

    f.render_widget(items_list, chunks[0]);

    let instructions = Paragraph::new(Line::from(vec![
        Span::raw("Use "),
        Span::styled("Up/Down", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" to move, "),
        Span::styled("Enter", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" to toggle, "),
        Span::styled("'c'", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" to Clean selected, "),
        Span::styled("'q'", Style::default().add_modifier(Modifier::BOLD)),
        Span::raw(" to quit."),
    ]))
    .block(Block::default().borders(Borders::ALL));
    f.render_widget(instructions, chunks[1]);

    // Render Modals
    match &app.modal {
        ModalState::SudoPrompt { input } => {
            let area = centered_rect(60, 20, size);
            f.render_widget(Clear, area); // clear background
            
            let masked_input: String = input.chars().map(|_| '*').collect();
            let prompt = Paragraph::new(format!("Sudo required for some tasks.\nPassword: {}\n\nPress Enter to execute.", masked_input))
                .block(Block::default().title(" Privilege Escalation ").borders(Borders::ALL).style(Style::default().fg(Color::Red)));
            f.render_widget(prompt, area);
        }
        ModalState::Success { freed_bytes } => {
            let area = centered_rect(40, 20, size);
            f.render_widget(Clear, area);
            
            let msg = Paragraph::new(format!("Successfully freed {} bytes!\n\nPress Enter to close.", freed_bytes))
                .block(Block::default().title(" Success ").borders(Borders::ALL).style(Style::default().fg(Color::Green)));
            f.render_widget(msg, area);
        }
        ModalState::Error { message } => {
            let area = centered_rect(60, 30, size);
            f.render_widget(Clear, area);
            
            let msg = Paragraph::new(format!("Error:\n{}\n\nPress Enter to close.", message))
                .block(Block::default().title(" Error ").borders(Borders::ALL).style(Style::default().fg(Color::Red)));
            f.render_widget(msg, area);
        }
        _ => {}
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
