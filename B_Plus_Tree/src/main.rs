use std::io;
use termion;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::event::Key;
use termion::screen::AlternateScreen;
use std::io::{Write,stdout};
use termion::cursor;

use tui::Terminal;
use tui::backend::TermionBackend;

use tui::text::{Span};
use tui::widgets::{Widget, Block, Borders, List, ListItem, StatefulWidget,ListState};
use tui::layout::{Layout, Constraint, Direction, Alignment,Margin, Rect};
use tui::style::{Style,Modifier, Color};
use tui::buffer::{Buffer};
use std::process;
pub mod b_plus_tree;


struct Events {
    items: Vec<String>,
    state: ListState
}


impl Events {
    fn new(items: Vec<String>) -> Events {
        Events {
            items,
            state: ListState::default(),
        }
    }

    pub fn set_items(&mut self, items: Vec<String>) {
        self.items = items;
        self.state = ListState::default();
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self){
        self.state.select(None);
    }

    pub fn get_selected(&mut self) -> usize {
        let i = match self.state.selected() {
            Some(i) => {
                i + 1
            }
            None => 0 
        };
        return i;
    }
}




fn main() -> Result<(), io::Error> { 
    let stdout = io::stdout().into_raw_mode()?;
    let backend = TermionBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    // let mut screen = AlternateScreen::from(stdout());
    
    let mut events = Events::new(vec![
        String::from("Create a B+ Tree"),
        String::from("Quit")
    ]);
    let mut stdin = termion::async_stdin().keys();

    let mut in_menu = true;
    let mut input_string : String = String::new();
    let mut x_cursor_pos = 4;

    loop {
        let input = stdin.next();
        if let Some(Ok(key)) = input {
            if in_menu {
                match key {
                    Key::Up => {
                        events.previous();
                    },
                    Key::Down =>{
                        events.next();
                    },
                    Key::Char('q') => {
                        terminal.clear();
                        break Ok(());
                    },
                    Key::Char('\n') => {
                        let i = events.get_selected();
                        if i == 1{
                            in_menu=false;
                        }else if i == 2{
                            terminal.clear();
                            break Ok(());
                        }
                    },
                    _ => {
                        //Do Nothing
                    }
                }
            } else {
                // let mut b_plus_tree = Bj
                let mut stdout_2 = io::stdout().into_raw_mode()?;
                match key {
                    Key::Ctrl('c') => {
                        terminal.clear();
                        break Ok(());
                    },
                    Key::Char('\n') => {
                        b_plus_tree::new("test",3);
                        //Do nothing for now.
                    },
                    _ => {
                        if  let termion::event::Key::Char(k) = key {
                            // write!(stdout_2, "\\{}\\",k);
                            input_string.push(k);
                            // write!(stdout_2,"\r");
                            // write!()
                            // write!(stdout_2,"{}",);
                            x_cursor_pos=x_cursor_pos+1;
                            write!(stdout_2,"{}{}{}", cursor::Goto(5,4),cursor::BlinkingBlock,input_string);

                            // cursor::Right(50q);
                            // write!(stdout_2,)
                            
                            // input_string=input_string.push(k);

                            // write!(stdout_2, "{}", input_string).unwrap();
                        //     stdout_2.lock().flush().unwrap();
                        }
                    }
                }
            } 
        }
        terminal.draw(|f| {
            if in_menu == true {            
                let items: Vec<ListItem>= events.items.iter().map(|i| ListItem::new(i.as_ref())).collect();
            
                let list = List::new(items)
                    .block(Block::default().title("Menu").borders(Borders::ALL))
                    .style(Style::default().fg(Color::White))
                    .highlight_style(Style::default().add_modifier(Modifier::ITALIC))
                    .highlight_symbol(">>");
                f.render_stateful_widget(list, f.size(), &mut events.state);

            } else{
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .margin(1)
                    .constraints(
                        [
                            Constraint::Percentage(10),
                            Constraint::Percentage(95),
                            Constraint::Percentage(5)
                        ].as_ref()
                    )
                    .split(f.size());
                // backend    
                
                let block = Block::default()
                    .title("Enter value to be inserted into B+ tree")
                    .borders(Borders::ALL);
                f.render_widget(block, chunks[0]);
                let block = Block::default()
                    .title("B+ Tree Viewer")
                    .borders(Borders::ALL);
                
                f.render_widget(block, chunks[1]);
                // if x_cursor_pos == 2 {

                    f.set_cursor(x_cursor_pos,3);
                // }                    
            }
        });

        // terminal.set_cursor(4,4);
        // terminal.show_cursor()

        // for c in stdin.keys() {
        //     match c.unwrap(){
        //         Key::Up => {
        //             events.previous();
        //         }
        //         Key::Down => {
        //             events.next();
        //         } 
        //     }
        // }   
        // In response to some input events or an external http request or whatever:
        // events.next();
    }
}
