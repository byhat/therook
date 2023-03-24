import QtQuick
import QtQuick.Controls
import QtQuick.Controls.Material
import QtQuick.Controls.Material.impl

Control {
    property int radius: 0

    id: pane

    background: Rectangle {
        color: Material.backgroundColor
        radius: pane.radius

        layer.enabled: pane.Material.elevation > 0
        layer.effect: ElevationEffect {
            elevation: pane.Material.elevation
        }
    }
}
