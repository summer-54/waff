use crossterm::event::{self, Event, KeyCode, KeyModifiers};
use ratatui::{text::{Line, Text}, widgets::{Block, Borders, List}, Frame};

use crate::{Instance, defaults::INSTANCE_FOLDER};

fn draw(frame: &mut Frame, instance: &Instance) {
    todo!();
    let text = Text::raw("Hollow");
    let lines = [Line::raw("A"), Line::raw("B")];
    let list = List::new(lines);
    let block = Block::default().borders(Borders::all()).title("Results");
    let block_inner = block.inner(frame.area());
    frame.render_widget(block, frame.area());
    //let bar = Bar::default().label(Line::raw("contəst")).text_value("Polygon contoest".to_string());
    //let bar_chart = BarChart
    frame.render_widget(text, frame.area());
    frame.render_widget(list, block_inner);
}

pub async fn start() -> Result<String, String> {
    let instance = Instance::get_from_dir(&INSTANCE_FOLDER.to_string()).await?;
    let mut terminal = ratatui::init();
    let result = loop {
        if let Err(err) = terminal.draw(|frame| draw(frame, &instance)) {
            break Err(format!("Failed to draw a frame: {err}"));
        }
        if event::read().is_ok_and(|event| event.as_key_event().is_some_and(|key_event| key_event.is_press() && key_event.modifiers.contains(KeyModifiers::CONTROL) && matches!(key_event.code, KeyCode::Char('q')))) {
            break Ok(format!("Tui succesfuly closed by pressing on [ -q- ]."));
        }
    };

    ratatui::restore();
    return result;
}
