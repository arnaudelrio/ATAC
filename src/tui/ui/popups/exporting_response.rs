use ratatui::Frame;
use ratatui::style::Style;
use ratatui::widgets::{Block, Borders, Clear, Paragraph};
use crate::app::app::App;
use crate::app::files::theme::THEME;
use crate::tui::utils::centered_rect::centered_rect;
use crate::tui::utils::stateful::text_input::SingleLineTextInput;

impl App<'_> {
    pub fn render_exporting_response_popup(&mut self, frame: &mut Frame) {
        let popup_block = Block::default()
            .title("Export response body")
            .borders(Borders::ALL)
            .style(Style::default().bg(THEME.read().ui.main_background_color));

        let area = centered_rect(60, 3, frame.area());

        let input = self.export_response_input.to_string();
        let input_paragraph = Paragraph::new(input.as_str())
            .block(Block::default().borders(Borders::ALL).title("File path"));

        frame.render_widget(Clear, area);
        frame.render_widget(popup_block, area);
        frame.render_widget(input_paragraph, area);

        self.export_response_input.display_cursor = true;
        
        frame.render_widget(SingleLineTextInput(&mut self.export_response_input), area);
    }
}
