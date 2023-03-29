import QtQuick
import QtQuick.Controls
import QtQuick.Layouts
import QtQuick.Controls.Material

RoundPane {
    Material.elevation: 6

    radius: 8
    padding: 0

    contentItem: RowLayout {
        spacing: 0

        EvalBar {
            Layout.preferredWidth: 8
            Layout.fillHeight: true

            evaluation: -20
        }

        ColumnLayout {
            Layout.fillWidth: true
            Layout.fillHeight: true
            Layout.margins: 8

            spacing: 8

            PlayerInfo {
                Layout.fillWidth: true

                avatarUri: "qrc:/chess/pieces/bK.svg"
                username: "black"
                rating: 401
            }

            Item {
                Layout.fillWidth: true
                Layout.fillHeight: true
            }

            PlayerInfo {
                Layout.fillWidth: true

                avatarUri: "qrc:/chess/pieces/wK.svg"
                username: "white"
                rating: 400
            }
        }
    }
}
