use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    DefaultTerminal, Frame,
    buffer::Buffer,
    layout::{Constraint, Flex, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Clear, Paragraph, Widget},
};
use std::io;

mod db;
mod models;
mod schema;

#[derive(Debug, Default)]
pub struct App {
    income: i32,
    spendings: i32,
    exit: bool,

    show_input_modal: bool,
    valueInput: i32,
    name: String,
    date: String,
    modal_type: String,
}

impl App {
    // runs the application main loop
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.draw(frame))?;
            self.handle_events()?;
        }

        Ok(())
    }

    fn draw(&self, frame: &mut Frame) {
        let area = frame.area();
        frame.render_widget(self, frame.area());

        if self.show_input_modal {
            let block = Block::bordered().title(format!("Add new {}", self.modal_type));
            let area = popup_area(area, 60, 20);
            frame.render_widget(Clear, area);
            frame.render_widget(block, area);
        }
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };

        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char(' ') => self.open_income_modal(),
            KeyCode::Backspace => self.open_spending_modal(),
            KeyCode::Esc => self.close_modal(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn close_modal(&mut self) {
        self.show_input_modal = false;
    }

    fn open_income_modal(&mut self) {
        self.modal_type = String::from("income");
        self.show_input_modal = true;
    }

    fn open_spending_modal(&mut self) {
        self.modal_type = String::from("spending");
        self.show_input_modal = true;
    }
}

fn popup_area(area: Rect, percent_x: u16, percent_y: u16) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);
    area
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Overall Balance ".bold());
        let instructions = Line::from(vec![
            " Add Income ".into(),
            "<SpaceBar>".blue().bold(),
            " Add Spending ".into(),
            "<BackSpace>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);

        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let counter_text = Text::from(vec![Line::from(vec![
            "Income: ".into(),
            self.income.to_string().green().bold(),
            " Spendings: ".into(),
            self.spendings.to_string().red().bold(),
        ])]);

        Paragraph::new(counter_text)
            .centered()
            .block(block)
            .render(area, buf)
    }
}

fn main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}
