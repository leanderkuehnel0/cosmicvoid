#!/bin/sh
wal -i "$(zenity --file-selection)"  --backend colorz
cd "$(dirname $0)"
cp $HOME/.cache/wal/pywal.kvconfig $HOME/.config/Kvantum/pywal/pywal.kvconfig
./obsidian.sh /home/andi/Documents/obsidian_vault
