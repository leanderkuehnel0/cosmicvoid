use serde::{Deserialize, Serialize};
use swayipc::{Connection, EventType, Event, Fallible, WindowChange, NodeType, Node, WorkspaceChange};
//use std::time::{Duration, SystemTime};

#[derive(Debug, Serialize, Deserialize)]
struct Scratch {
	len: usize,
	focused: bool,
	urgent: bool,
	#[serde(skip_serializing)]
	windows: Vec::<i64>,
	floating: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct Out {
	scratch: Scratch,
	ws: [Option<Workspace>;9],
//	#[serde(skip_serializing)]
//	last_update: SystemTime,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
struct Workspace {
	output: String,
	focused: bool,
	urgent: bool,
}

impl Out {
  fn print(&self) {
      println!("{}", serde_json::to_string(self).unwrap());
  }
  fn remove_window(&mut self,id: i64 ) {
  		let index = self.scratch.windows.iter().position(|x| *x == id);
		if let Some(index) = index {
			self.scratch.windows.remove(index);
		}
  }
	fn add_window(&mut self, windows: Vec::<i64>) {
		for window in windows {
			if !self.scratch.windows.contains(&window) {
				self.scratch.windows.push(window)
			}
		}
	}
}

fn get_focused_window_node(sway: &mut Connection) -> Option<Node> {
	sway.get_tree().unwrap().find_focused(|x| ( x.node_type == NodeType::Con || x.node_type == NodeType::FloatingCon ) && x.name.is_some())
}

impl Out {
	fn update(&mut self, sway: &mut Connection, event: Option<Event>) {
//		if self.last_update.elapsed().unwrap() >= Duration::from_millis(20) {
//			return
//		}
		{
			let windows = sway.get_tree().unwrap().nodes[0].nodes[0].floating_nodes.iter().map(|x| x.id).collect::<Vec<i64>>();
//			let focused_id: Option<i64> = get_focused_window_node(sway).and_then(|x| Some(x.id));
			if let Some(event) = event {
				match event {
					Event::Workspace(event) => 
						if event.change == WorkspaceChange::Init || event.change == WorkspaceChange::Rename {
							return
						}
					Event::Window(event) => 
						if event.change == WindowChange::Focus {
							if self.scratch.len != 0 {
								let focused_node = get_focused_window_node(sway);
								if let Some(focused_node) = focused_node {
									self.scratch.focused = self.scratch.windows.contains(&focused_node.id)
								}
							} else {
								return
							}
						} else if event.change == WindowChange::Urgent {
							if self.scratch.len != 0 {
								if self.scratch.windows.contains(&event.container.id) {
		                            self.scratch.urgent = event.container.urgent;
		                            self.print();
		                        }
							} else {
								return
							}
						} else if event.change == WindowChange::Close && self.scratch.len != 0 {
							self.remove_window(event.container.id);
						} else if event.change == WindowChange::Move {
							self.add_window(windows)
						},
					Event::Binding(event) =>
						if self.scratch.focused && event.binding.command == "floating toggle" {
							self.remove_window(
								get_focused_window_node(sway).unwrap().id
							);
						}
					_ => (),
				};
			} else {
				self.add_window(windows);
			}
			self.scratch.len = self.scratch.windows.len();
		}
		{
			let workspaces = sway.get_workspaces().unwrap();
			let mut out = [None,None,None,None,None,None,None,None,None];
			for ws in workspaces {
				out[ws.num as usize] = Some(Workspace {
					output: ws.output,
					focused: ws.focused,
					urgent: ws.urgent,
				})
			}
			self.ws = out;
		}
//		self.last_update = SystemTime::now();
//		println!("{:#?}", self)
		self.print();
	}
}

fn main() -> Fallible<()> {
    let mut out = Out { 
    	scratch: Scratch {
    		len: 0,
    		focused: false,
    		urgent: false,
    		windows: Vec::new(), 
    		floating: false,
	    },
	    ws: [None,None,None,None,None,None,None,None,None],
//    	last_update: SystemTime::now()
	  };
    let mut sway = Connection::new()?;
    out.update(&mut sway, None);

    for event in Connection::new()?.subscribe(
        [
        	EventType::Window,
        	EventType::Workspace,
        	EventType::Output,
        	EventType::Binding,
        	EventType::Shutdown
        ]
    )? {
    		let event = event?;
        match event {
            Event::Shutdown(_) => std::process::exit(0),
            event => out.update(&mut sway, Some(event)),
        }
    }
    Ok(())
}
