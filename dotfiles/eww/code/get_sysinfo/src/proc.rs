use super::*;

pub struct Proc {
    pub pid: u64,
    pub name: String,
    pub cpu: f32,
    pub mem: f32,
    pub threads: u64,
}

pub fn new() {
	match glob("/proc/*") {
		Ok(glob) => {
			for entry in glob {
				match entry {
					Ok(path) => {
				        let comm = fs::read_to_string(path.join("comm"));
						let comm = match comm {
						    Ok(comm) => comm,
						    Err(_) => continue
						};
						let comm = comm.trim();
						let comm = comm.to_string();
						let cpu = fs::read_to_string(path.join("stat"));
						let x = match cpu {
						    Ok(cpu) => cpu,
						    Err(_) => continue,
						};
						let mut x = x.split_whitespace();
						let cpu = ( x.nth(14).unwrap().parse::<f32>().unwrap() + x.next().unwrap().parse::<f32>().unwrap() ) / 100.0;
						println!("{}: {}", comm, cpu)
					}
            		Err(_) => (),
				}
			}
		}
   		Err(err) => println!("{err}"),
	}
}

// for i in /proc/*; do echo $( cat $i/comm): $(( ( $(cat $i/stat | cut -d " " -f 14) + $(cat $i/stat | cut -d " " -f 14) )/ 100)); done | grep -v 0              [0]
