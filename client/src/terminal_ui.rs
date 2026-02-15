use crossterm::event::{self, KeyCode, KeyModifiers};
use ratatui::{Frame, text::{Line, Text}, widgets::{Block, Borders, List}};
use anyhow::{Result, Context, anyhow};

use lib::{instance::Instance, defaults::INSTANCE_FOLDER};

fn draw(frame: &mut Frame, instance: &Instance) {
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

async fn app(mut terminal: ratatui::DefaultTerminal) -> Result<Box<str>> {
    let instance = Instance::get_from_dir(&INSTANCE_FOLDER.to_string()).await.context("while loading Instance from folder")?;
    loop {
        if let Err(err) = terminal.draw(|frame| draw(frame, &instance)) {
            return Err(anyhow!("Failed to draw a frame: {err}"));
        }
        if event::read().is_ok_and(|event| event.as_key_event().is_some_and(|key_event| key_event.is_press() && key_event.modifiers.contains(KeyModifiers::CONTROL) && matches!(key_event.code, KeyCode::Char('q')))) {
            return Ok(format!("Tui succesfuly closed by pressing on [ -q- ].").into());
        }
    };
}

pub async fn start() -> Result<Box<str>> {
    let terminal = ratatui::init();
    let result = app(terminal).await;
    ratatui::restore();
    result.context("while working tui")
}
