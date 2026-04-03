import QtQuick 2.15

QtObject {
    id: wal

    property var colors: ({})
    property string background: "#000000"
    property string foreground: "#ffffff"
    property string accent: "#5e81ac"

    function load() {
        const path = "/home/" + Qt.platform.os === "linux"
            ? Qt.platform.user
            : "" + "/.cache/wal/colors.json"

        const xhr = new XMLHttpRequest()
        xhr.open("GET", path, false)
        xhr.send()

        if (xhr.status === 200) {
            const data = JSON.parse(xhr.responseText)
            colors = data.colors
            background = data.special.background
            foreground = data.special.foreground
            accent = data.colors.color4
        }
    }

    Component.onCompleted: load()
}

