#!/bin/python
def hex_to_rgb(hex):
  return 'rgba' + str(tuple(int(hex[i:i+2], 16) for i in (0, 2, 4)) + tuple([0.8])) + ';'

with open("/home/andi/.cache/wal/colors") as f_in:
    col = f_in.readline().strip().strip('#')
    print(hex_to_rgb(col))
    with open("/home/andi/.cache/wal/colors-waybar.css", "a") as f_out:
        f_out.write("\n@define-color background " + hex_to_rgb(col))
