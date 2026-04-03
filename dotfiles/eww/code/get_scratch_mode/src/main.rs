use serde::{Deserialize, Serialize};
use swayipc::{Connection, EventType, WindowChange, Event, Fallible};
use std::time::{Duration, SystemTime};

#[derive(Debug, Serialize, Deserialize)]
struct Scratch {
    len: usize,
    focused: bool,
    urgent: bool,
	#[serde(skip_serializing)]
    windows: Vec::<i64>,
    //last_update: SystemTime,
}

#[derive(Debug, Serialize, Deserialize)]
struct Out {
    scratch: Scratch,
    mode: Option<String>,
}

impl Out {
    fn print(&self) {
        println!("{}", serde_json::to_string(self).unwrap());
    }
}

impl Scratch {
    fn update(&mut self) {
        self.len = self.windows.len();
        //self.last_update = SystemTime::now();
    }
    fn get_windows(&mut self) -> Fallible<()> {
        self.windows = Connection::new()?.get_tree()?.nodes[0].nodes[0].floating_nodes.iter().map(|x| x.id).collect::<Vec<i64>>();
        self.update();
        Ok(())
    }
    fn rm(&mut self, num: i64) {
        self.windows.retain(|x| *x != num);
        self.update();
    }
    fn set_focused(&mut self, x: bool) {
        self.focused = x;
        self.update();
    }
    fn set_urgent(&mut self, x: bool) {
        self.urgent = x;
        self.update();
    }
}



fn main() -> Fallible<()> {
    let mut out = Out { scratch: Scratch {len: 0, focused: false, urgent: false, windows: Vec::new(), /*last_update: SystemTime::now()*/ }, mode: None};
    out.scratch.get_windows()?;
    out.print();
    for event in Connection::new()?.subscribe(
        [EventType::Window,EventType::Binding,EventType::Mode,EventType::Shutdown]
    )? {
        match event? {
            Event::Shutdown(_) => std::process::exit(0),
            Event::Window(event) => {
                match event.change {
                    WindowChange::Close | WindowChange::Floating => {
                        if out.scratch.windows.contains(&event.container.id) {
                            out.scratch.rm(event.container.id);
                            out.scratch.set_focused(false);
                            out.print();
                        }
                    },
                    WindowChange::Focus => {
                        if out.scratch.windows.contains(&event.container.id) {
                            out.scratch.set_focused(true);
                            out.print();
                        } else if out.scratch.focused {
                            out.scratch.set_focused(false);
                            out.print();
                        }
                    },
                    WindowChange::Urgent => {
                        if out.scratch.windows.contains(&event.container.id) {
                            out.scratch.set_urgent(event.container.urgent);
                            out.print();
                        }
                    },
                    _ => ()
                }
            },
            Event::Binding(binding) => {
                if &binding.binding.command == "move scratchpad" {
                    out.scratch.get_windows()?;
                    out.print();
                }
            },
            Event::Mode(mode) => {
                if mode.change == "default" {
                    out.mode = None;
                } else {
                    out.mode = Some(mode.change);
                }
                out.print();
            }
            _ => (),
        }
    }
    Ok(())
}
