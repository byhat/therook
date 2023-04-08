import QtQuick
import QtQuick.Controls
import QtQuick.Layouts

RowLayout {
    required property var avatarUri
    required property string username
    required property int rating

    Image {
        Layout.preferredWidth: 48
        Layout.preferredHeight: 48

        source: avatarUri
        sourceSize.width: 48
        sourceSize.height: 48
    }

    ColumnLayout {
        RowLayout {
            spacing: 2

            Label {
                text: username

                font.pointSize: 11
                font.bold: true
                font.letterSpacing: 1
            }

            Label {
                text: "(" + rating + ")"

                font.pointSize: 9
            }
        }

        Label {
            text: "have pieces"

            font.pointSize: 10
        }
    }
}
