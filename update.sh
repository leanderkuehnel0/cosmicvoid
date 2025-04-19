#!/bin/bash
cd $(dirname $0)
cp -rf dotfiles/* $HOME/.config/
swaymsg reload
