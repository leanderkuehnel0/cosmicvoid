# Keybinds Cheatsheet

---

## Programs & Applications

| Keybind | Action |
|---------|--------|
| `$mod + Return` | Open terminal (foot) |
| `$mod + b` | Open browser (Firefox) |
| `$mod + o` | Open Obsidian |
| `$mod + e` | Open file manager (Thunar) |
| `$mod + Shift + Return` | Open application menu (wofi) |
| `$mod + l` | Lock screen (swaylock) |

---

## Window Management

| Keybind | Action |
|---------|--------|
| `$mod + q` | Kill focused window |
| `$mod + f` | Toggle fullscreen |
| `$mod + Shift + space` | Toggle floating mode |
| `$mod + space` | Focus toggle (tiling/floating) |
| `$mod + a` | Focus parent container |

---

## Navigation

### Focus Movement
| Keybind | Action |
|---------|--------|
| `$mod + Left` / `$mod + h` | Focus left |
| `$mod + Down` / `$mod + j` | Focus down |
| `$mod + Up` / `$mod + k` | Focus up |
| `$mod + Right` / `$mod + l` | Focus right |

### Window Movement
| Keybind | Action |
|---------|--------|
| `$mod + Shift + h` | Move left |
| `$mod + Shift + j` | Move down |
| `$mod + Shift + k` | Move up |
| `$mod + Shift + l` | Move right |
| `$mod + Shift + Left/Down/Up/Right` | Move with arrow keys |

---

## Workspaces

### Switch to Workspace
| Keybind | Action |
|---------|--------|
| `$mod + 1` - `$mod + 0` | Switch to workspace 1-10 |
| `$mod + Mouse Wheel Up` | Previous workspace |
| `$mod + Mouse Wheel Down` | Next workspace |

### Move Window to Workspace
| Keybind | Action |
|---------|--------|
| `$mod + Shift + 1` - `$mod + Shift + 0` | Move to workspace 1-10 |

---

## Layout Management

| Keybind | Action |
|---------|--------|
| `$mod + s` | Stack layout |
| `$mod + w` | Tabbed layout |
| `$mod + j` | Toggle split layout |

---

## Resizing Mode

Enter resize mode with: `$mod + r`

| Keybind | Action |
|---------|--------|
| `h` or `Left` | Shrink width |
| `j` or `Down` | Grow height |
| `k` or `Up` | Shrink height |
| `l` or `Right` | Grow width |
| `Return` or `Escape` | Exit resize mode |

---

## Scratchpad

| Keybind | Action |
|---------|--------|
| `$mod + Shift + -` | Move window to scratchpad |
| `$mod + -` | Show/hide scratchpad window |

---

## Utilities & Media

| Keybind | Action |
|---------|--------|
| `$mod + Shift + c` | Reload Sway config |
| `$mod + Shift + p` | Take screenshot (grim + slurp) |
| `$mod + v` | Open clipboard history (wofi) |
| `$mod + Shift + w` | Change wallpaper (pywal) |
| `$mod + Shift + e` | Exit Sway |
| `Print` | Take full screenshot |

### Volume Control
| Keybind | Action |
|---------|--------|
| `XF86AudioMute` | Toggle mute |
| `XF86AudioRaiseVolume` | Volume +5% |
| `XF86AudioLowerVolume` | Volume -5% |
| `XF86AudioMicMute` | Toggle microphone mute |

### Brightness Control
| Keybind | Action |
|---------|--------|
| `XF86MonBrightnessUp` | Brightness +5% |
| `XF86MonBrightnessDown` | Brightness -5% |

---

## Touchpad Gestures

| Gesture | Action |
|---------|--------|
| Swipe Right | Previous workspace |
| Swipe Left | Next workspace |
| Swipe Up | Open application menu |

---

## Notes

- Most keybinds use `$mod` (Super/Win key)
- Arrow keys can be used as alternatives to hjkl navigation
- Some utilities use special keys (XF86*) for media control
- Use `$mod + Shift + e` to safely exit Sway
- Configuration includes auto-starting: Waybar, PipeWire, Wireplumber, Pywalfox, and Cliphist
