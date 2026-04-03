import QtQuick
import QtQuick.Layouts

import Quickshell
import Quickshell.Wayland
import Quickshell.Ipc.Sway

ShellWindow {
    id: bar
    height: 32
    anchors.top: true
    exclusiveZone: height
    color: wal.background

    WalColors { id: wal }

    SwayIpc {
        id: sway
    }

    RowLayout {
        anchors.fill: parent
        anchors.margins: 8
        spacing: 12

        // ── Workspaces (IPC) ─────────────────────
        RowLayout {
            spacing: 6

            Repeater {
                model: sway.workspaces

                Rectangle {
                    width: 20
                    height: 20
                    radius: 10
                    color: modelData.focused ? wal.accent : "transparent"
                    border.color: wal.foreground
                    border.width: 1

                    Text {
                        anchors.centerIn: parent
                        text: modelData.name
                        font.pixelSize: 10
                        color: modelData.focused
                               ? wal.background
                               : wal.foreground
                    }

                    MouseArea {
                        anchors.fill: parent
                        onClicked: sway.command(
                            "workspace " + modelData.name
                        )
                    }
                }
            }
        }

        Item { Layout.fillWidth: true }

        // ── Clock (no services needed) ───────────
        Text {
            id: clock
            font.pixelSize: 12
            color: wal.foreground

            Timer {
                interval: 1000
                running: true
                repeat: true
                onTriggered: {
                    const d = new Date()
                    clock.text = d.toLocaleTimeString([], {
                        hour: "2-digit",
                        minute: "2-digit"
                    })
                }
            }
        }
    }
}

