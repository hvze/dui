use types::Architecture;
use termion::event;
use termion::input::TermRead;
use std::sync::mpsc;
use std::thread;
use std::io;
use tui::backend::MouseBackend;
use tui::layout::{Direction, Group, Rect, Size};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Item, List, SelectableList, Widget};
use tui::Terminal;
use input;

pub enum Event {
    Input(event::Key)
}

fn draw(t: &mut Terminal<MouseBackend>, arch: &Architecture) {
    Group::default()
        .direction(Direction::Horizontal)
        .sizes(&[Size::Percent(30), Size::Percent(70)])
        .render(t, arch.get_size(), |t, chunks| {
            Group::default()
                .direction(Direction::Vertical)
                .sizes(&[Size::Percent(30), Size::Percent(70)])
                .render(t, &chunks[0], |t, chunks| {
                    SelectableList::default()
                        .block(Block::default().borders(Borders::ALL).title(&*arch.display_servers()))
                        .items(&arch.get_servers_for_display())
                        .select(arch.get_current_server_index().unwrap());
                })
        });
}

pub fn draw_ui(arch: Architecture) {
    let backend = MouseBackend::new().unwrap();
    let mut term = Terminal::new(backend).unwrap();
    let (snd, rcv) = mpsc::channel();
    let inp_snd = snd.clone();

    // spawn input thread
    thread::spawn(move || {
        let stdin = io::stdin();
        for c in stdin.keys() {
            let evt = c.unwrap();
            inp_snd.send(Event::Input(evt)).unwrap();
            if evt == event::Key::Char('q') {
                break;
            }
        }
    });

    term.clear().unwrap();
    term.hide_cursor().unwrap();

    {
        draw(&mut term, &arch);
    }

    loop {
        let evt = rcv.recv().unwrap();
        match evt {
            Event::Input(inp) => match inp {
                event::Key::Char('q') => break,
                _ => input::handle_input(&inp),
            }
        }
        draw(&mut term, &arch);
    }
    term.show_cursor().unwrap();
    term.clear().unwrap();
}