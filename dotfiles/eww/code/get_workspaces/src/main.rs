use serde::{Deserialize, Serialize};
use swayipc::Fallible;
use swayipc::{Connection, EventType, Event, WorkspaceChange};

#[derive(Debug, Serialize, Deserialize)]
struct Workspace {
    output: String,
    focused: bool,
    urgent: bool,
}

fn print(out: &[Option<Workspace>;6]) {
    println!("{}", serde_json::to_string(out).unwrap());
}

fn sway() -> Fallible<()> {
    let mut sway = Connection::new()?;
    let mut get_ws = || {
        let workspaces = sway.get_workspaces().unwrap();
        let mut out = [None,None,None,None,None,None];
        for ws in workspaces {
            out[ws.num as usize] = Some(Workspace {
                output: ws.output,
                focused: ws.focused,
                urgent: ws.urgent,
            })
        }
        print(&out);
    };
    get_ws();

    for event in Connection::new()?.subscribe([EventType::Workspace,EventType::Shutdown])? {
        match event? {
            Event::Shutdown(_) => std::process::exit(0),
            Event::Workspace(workspace) => {
                let change = workspace.change;
                if let WorkspaceChange::Focus = change {
                    get_ws();
                }
            },
            _ => std::process::exit(1),
        }
    }
    Ok(())
}

fn main() {
    sway().unwrap();
}
