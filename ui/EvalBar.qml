import QtQuick
import QtQuick.Layouts

Column {
    required property int evaluation // From -100 to 100

    readonly property int barHeight: height * (evaluation + 100) / 200

    Rectangle {
        width: parent.width
        height: parent.height - barHeight

        color: "#000000"
    }

    Rectangle {
        width: parent.width
        height: barHeight

        color: "#FFFFFF"
    }
}
