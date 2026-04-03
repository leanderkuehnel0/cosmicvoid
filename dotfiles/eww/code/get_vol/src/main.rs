use serde::Serialize;
use std::{
    process::{exit, Command},
    thread::sleep,
    time::Duration,
};
use swayipc::Fallible;
use swayipc::{Connection, Event, EventType, InputChange};


#[derive(Debug, Serialize)]
struct Out {
    vol: f32,
    muted: bool,
    list_names: Vec<String>,
}

impl Out {
    fn print(&self) {
        println!("{}", serde_json::to_string(self).unwrap());
    }
    fn get_volume(&mut self) -> Result<(), ()> {
        {
					let output = match Command::new("wpctl")
					  .args(["status"])
					  .output()
					{
					  Ok(output) => output.stdout,
					  Err(_) => exit(1),
					};
					let output = String::from_utf8_lossy(&output);

					let output = output.lines();
					let mut output = output.skip_while(|x| !x.starts_with("Audio")).skip_while(|x| !x.contains("Sinks:")).take_while(|x| !x.contains("Sources:"));
					output.next().expect("hugh wierd");

					let mut output: Vec<&str> = output.collect();
					output.pop();

					self.list_names = Vec::with_capacity(1);

					for output in output {
						let mut default= false;
						let mut output = output.split_whitespace();
						output.next().unwrap();
						
						let mut i = output.next().unwrap();
						if i == "*" {
							default = true;
							i = output.next().unwrap();
						}
						let id = i[..(i.len()-1)].parse::<u64>().unwrap();

						let mut name: String = String::new();
						let mut temp = false;
						for i in output {
							if i == "[vol:" {
								if default {
									temp = true;
									continue
								} else {
									break
								}
							}
							if !temp {
								name += " ";
								name += i;
							} else {
								if i.ends_with(']') {
									self.vol = i[..(i.len() -1)].parse().unwrap();
									self.muted = false
								} else {
									self.vol = i.parse().unwrap();
									self.muted = true
								}
								break
							}
						}
						self.list_names.push(format!("{} ({})", name.trim(), id));
					}
        }
        self.print();
        Ok(())
    }
}

fn main() -> Fallible<()> {
    let mut out = Out {
        vol: 0.0,
        muted: false,
        list_names: Vec::with_capacity(1),
    };
    while out.get_volume().is_err() {
        sleep(Duration::from_millis(20))
    }
    let sway = Connection::new()?;
    for event in sway.subscribe([EventType::Binding, EventType::Input, EventType::Shutdown])? {
        match event? {
            Event::Shutdown(_) => std::process::exit(0),
            Event::Binding(binding) => {
                if binding.binding.command.starts_with("exec swayosd-client") {
                    sleep(Duration::from_millis(60));
                    while out.get_volume().is_err() {
                        sleep(Duration::from_millis(20))
                    }
                }
            }
            Event::Input(input) => {
            	if let InputChange::Added | InputChange::Removed = input.change {
           			sleep(Duration::from_millis(60));
                 while out.get_volume().is_err() {
                     sleep(Duration::from_millis(20))
                 }
            	}
            }
            _ => (),
        }
    }
    Ok(())
}

