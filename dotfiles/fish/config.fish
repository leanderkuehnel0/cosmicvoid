if status is-interactive
	cat ~/.cache/wal/sequences
	# Commands to run in interactive sessions can go here
end
set editor nvim
export PATH=$PATH:$HOME/.cargo/bin/
fish_config theme choose "Dracula Official"
