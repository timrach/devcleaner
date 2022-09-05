use super::app::App;
use crate::{app::Screen, components::StatefulList};
use tui::{
    backend::Backend,
    layout::{Constraint, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Span,
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    match app.current_screen {
        Screen::Overview => draw_main(f, app),
        Screen::Confirmation => draw_confirmation_dialogue(f, app),
    }
}

pub fn draw_confirmation_dialogue<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = build_layout(f.size());
    let message = Paragraph::new(format!(
        "Do you want do move {} folders to the bin?",
        app.scan_results.marked_items.len()
    ));
    f.render_widget(message, chunks[1]);

    let legend = Paragraph::new("ENTER to confirm deletion, ESC to go back");
    f.render_widget(legend, chunks[2]);
}

pub fn draw_main<B: Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = build_layout(f.size());
    let header = build_header(app.path.as_str());
    let results = build_result_list(&app.scan_results);
    let footer = build_footer();

    f.render_widget(header, chunks[0]);
    f.render_stateful_widget(results, chunks[1], &mut app.scan_results.state);
    f.render_widget(footer, chunks[2]);
}

pub fn build_layout(size: Rect) -> Vec<Rect> {
    Layout::default()
        .margin(2)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(2),
            Constraint::Length(1),
        ])
        .split(size)
}

pub fn build_header(search_path: &str) -> Paragraph {
    Paragraph::new(format!(
        "Looked for dev dependencies in: \t {}",
        search_path
    ))
}

pub fn build_footer() -> Paragraph<'static> {
    Paragraph::new(Span::raw(
        "q: quit, x: toggle entry, a: toggle all, d: move toggled to trash \t n: move 10 up, m: move 10 down",
    ))
}

fn build_list_entry(entry: &str, checked: bool) -> ListItem<'static> {
    let mark = if checked { "x" } else { " " };
    ListItem::new(format!("[{}] \t {}", mark, entry))
}

pub fn build_result_list(list_data: &StatefulList<String>) -> List<'static> {
    let block = Block::default().borders(Borders::ALL).title(format!(
        "Scan Results ({}/{})",
        list_data.state.selected().unwrap_or(0) + 1,
        list_data.items.len()
    ));

    let items: Vec<_> = list_data
        .items
        .iter()
        .map(|x| build_list_entry(x, list_data.is_marked(x)))
        .collect();

    List::new(items).block(block).highlight_style(
        Style::default()
            .bg(Color::Yellow)
            .add_modifier(Modifier::BOLD),
    )
}
