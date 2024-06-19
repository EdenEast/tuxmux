use crossterm::event::{KeyEvent, KeyModifiers};
use miette::{IntoDiagnostic, Result};

use std::sync::Arc;
use std::time::Duration;

use crossterm::event::KeyCode;
use nucleo::{
    pattern::{CaseMatching, Normalization},
    Injector, Nucleo,
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style, Stylize},
    text::{Line, Span},
    widgets::{
        block::Position, Block, Borders, HighlightSpacing, List, ListDirection, ListItem,
        ListState, Paragraph,
    },
    Frame, Terminal,
};

use crate::config::Config;

use super::event::{Event, EventHandler};
use super::tui::Tui;

pub struct Picker {
    pub injector: Injector<String>,
    matcher: Nucleo<String>,
    selection: ListState,
    filter: String,
    cursor_pos: u16,
    prompt: String,
    should_exit: bool,
    select_single: bool,
}

impl Default for Picker {
    fn default() -> Self {
        Self::new()
    }
}

impl Picker {
    pub fn new() -> Self {
        let matcher = Nucleo::new(nucleo::Config::DEFAULT, Arc::new(request_redraw), None, 1);
        let injector = matcher.injector();

        Picker {
            injector,
            matcher,
            selection: ListState::default(),
            filter: String::default(),
            cursor_pos: 0,
            prompt: String::default(),
            should_exit: false,
            select_single: true,
        }
    }

    pub fn from_config(config: &Config) -> Self {
        let mut picker = Picker::new();
        picker.select_single = config.picker.select_single;
        picker
    }

    pub fn items(self, list: &[String]) -> Self {
        for str in list {
            self.injector
                .push(str.to_owned(), |_, dst| dst[0] = str.to_owned().into());
        }
        self
    }

    pub fn prompt(mut self, prompt: &str) -> Self {
        self.prompt = prompt.to_string();
        self
    }

    pub fn filter(mut self, query: Option<&str>) -> Self {
        if let Some(filter) = query {
            self.filter = filter.to_string();
        }
        self
    }

    pub fn set_select_only(mut self, value: bool) -> Self {
        self.select_single = value;
        self
    }

    pub fn select(mut self) -> Result<Option<String>> {
        // If we have set a pre-filter then have to update the state and check if there is only
        // one match
        if !self.filter.is_empty() {
            self.cursor_pos = self.filter.len() as u16;
            self.update_matcher_pattern(self.filter.clone().as_ref());

            if self.select_single {
                self.matcher.tick(10);
                let snapshot = self.matcher.snapshot();
                if snapshot.matched_item_count() == 1 {
                    let s = snapshot
                        .get_matched_item(0)
                        .expect("There is only one matched item");
                    return Ok(Some(s.data.to_string()));
                }
            }
        }

        let backend = CrosstermBackend::new(std::io::stderr());
        let terminal = Terminal::new(backend).into_diagnostic()?;
        let events = EventHandler::new(Duration::from_millis(15));
        let mut tui = Tui::new(terminal, events);
        tui.enter()?;

        let mut selection = None;
        while !self.should_exit {
            tui.draw(&mut self)?;
            selection = match tui.events.next()? {
                Event::Tick => None,
                Event::Key(key_event) => self.update(key_event),
            };
        }

        tui.exit()?;
        Ok(selection)
    }

    fn update(&mut self, key_event: KeyEvent) -> Option<String> {
        match key_event.code {
            KeyCode::Esc => self.should_exit = true,
            KeyCode::Enter => {
                if let Some(selection) = self.get_selected_text() {
                    self.should_exit = true;
                    return Some(selection);
                }
            }
            KeyCode::Backspace => self.backspace(),
            KeyCode::Delete => self.delete(),
            KeyCode::Up => self.move_cursor_up(),
            KeyCode::Down => self.move_cursor_down(),
            KeyCode::Left => self.move_cursor_left(),
            KeyCode::Right => self.move_cursor_right(),
            _ => {
                if let KeyCode::Char(c) = key_event.code {
                    if key_event.modifiers.contains(KeyModifiers::CONTROL) {
                        match c {
                            'c' | 'd' | 'z' => self.should_exit = true,
                            'p' => self.move_cursor_up(),
                            'n' => self.move_cursor_down(),
                            'b' | 'h' => self.move_cursor_left(),
                            'f' | 'l' => self.move_cursor_right(),
                            _ => {}
                        }
                    } else {
                        self.update_filter(c)
                    }
                }
            }
        };
        None
    }

    pub fn render(&mut self, frame: &mut Frame) {
        self.matcher.tick(10);
        let snapshot = self.matcher.snapshot();
        let matches = snapshot
            .matched_items(..snapshot.matched_item_count())
            .map(|item| ListItem::new(item.data.as_str()));

        if let Some(selected) = self.selection.selected() {
            if snapshot.matched_item_count() == 0 {
                self.selection.select(None);
            } else if selected > snapshot.matched_item_count() as usize {
                self.selection
                    .select(Some(snapshot.matched_item_count() as usize - 1));
            }
        } else if snapshot.matched_item_count() > 0 {
            self.selection.select(Some(0));
        }

        let border_color = Color::DarkGray;
        let info_color = Color::LightYellow;

        let table = List::new(matches)
            .direction(ListDirection::BottomToTop)
            .highlight_spacing(HighlightSpacing::Always)
            .highlight_symbol("> ")
            .highlight_style(Style::default().fg(Color::LightBlue))
            .block(
                Block::default()
                    .borders(Borders::BOTTOM)
                    .border_style(Style::default().fg(border_color))
                    .title_style(Style::default().fg(info_color))
                    .title_position(Position::Bottom)
                    .title(format!(
                        "{}/{}",
                        snapshot.matched_item_count(),
                        snapshot.item_count()
                    )),
            );

        let layout = Layout::new(
            Direction::Vertical,
            [
                Constraint::Length(frame.size().height - 1),
                Constraint::Length(1),
            ],
        )
        .split(frame.size());

        frame.render_stateful_widget(table, layout[0], &mut self.selection);

        let prompt = Span::from(&self.prompt).fg(Color::LightBlue).bold();
        let input_text = Span::raw(&self.filter);
        let input_line = Line::from(vec![prompt, input_text]);
        let input = Paragraph::new(vec![input_line]);
        frame.render_widget(input, layout[1]);
        frame.set_cursor(
            layout[1].x + self.cursor_pos + self.prompt.len() as u16,
            layout[1].y,
        );
    }

    fn get_selected_text(&self) -> Option<String> {
        if let Some(index) = self.selection.selected() {
            return self
                .matcher
                .snapshot()
                .get_matched_item(index as u32)
                .map(|item| item.data.to_owned());
        }

        None
    }

    fn move_cursor_up(&mut self) {
        let item_count = self.matcher.snapshot().matched_item_count() as usize;
        if item_count == 0 {
            return;
        }

        let max = item_count - 1;

        match self.selection.selected() {
            Some(i) if i >= max => {}
            Some(i) => self.selection.select(Some(i + 1)),
            None => self.selection.select(Some(0)),
        }
    }

    fn move_cursor_down(&mut self) {
        match self.selection.selected() {
            Some(0) => {}
            Some(i) => self.selection.select(Some(i - 1)),
            None => self.selection.select(Some(0)),
        }
    }

    fn move_cursor_left(&mut self) {
        if self.cursor_pos > 0 {
            self.cursor_pos -= 1;
        }
    }

    fn move_cursor_right(&mut self) {
        if self.cursor_pos < self.filter.len() as u16 {
            self.cursor_pos += 1;
        }
    }

    fn update_filter(&mut self, c: char) {
        if self.filter.len() == u16::MAX as usize {
            return;
        }

        let prev_filter = self.filter.clone();
        self.filter.insert(self.cursor_pos as usize, c);
        self.cursor_pos += 1;

        self.update_matcher_pattern(&prev_filter);
    }

    fn backspace(&mut self) {
        if self.cursor_pos == 0 {
            return;
        }

        let prev_filter = self.filter.clone();
        self.filter.remove(self.cursor_pos as usize - 1);

        self.cursor_pos -= 1;

        if self.filter != prev_filter {
            self.update_matcher_pattern(&prev_filter);
        }
    }

    fn delete(&mut self) {
        if (self.cursor_pos as usize) == self.filter.len() {
            return;
        }

        let prev_filter = self.filter.clone();
        self.filter.remove(self.cursor_pos as usize);

        if self.filter != prev_filter {
            self.update_matcher_pattern(&prev_filter);
        }
    }

    fn update_matcher_pattern(&mut self, prev_filter: &str) {
        self.matcher.pattern.reparse(
            0,
            self.filter.as_str(),
            CaseMatching::Smart,
            Normalization::Smart,
            self.filter.starts_with(prev_filter),
        );
    }
}

fn request_redraw() {}
