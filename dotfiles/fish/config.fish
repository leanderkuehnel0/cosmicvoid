if status is-interactive
	cat ~/.cache/wal/sequences
	# Commands to run in interactive sessions can go here
end
set editor nvim
alias "poweroff" "sudo poweroff"
alias "mount_ssd" "/home/andi/cosmic/scripts/mount_ssd"

fish_config theme choose "Dracula Official"
fish_add_path /home/andi/.local/bin
fish_add_path /home/andi/sdcard/miniconda3
fish_add_path /home/andi/.opencode/bin
