pkill waybar
export CONFIG_PATH="$HOME/.config/waybar/"
$CONFIG_PATH/pywal_background.py
sleep 2
pkill waybar
waybar -c $CONFIG_PATH/$1/$(cat $CONFIG_PATH/current_theme)/config -s $CONFIG_PATH/$1/$(cat $CONFIG_PATH/current_theme)/style.css >> $CONFIG_PATH/log
