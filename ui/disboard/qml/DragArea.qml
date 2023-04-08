import QtQuick

Item {
    property alias dragPos: dragController.curPos

    id: dragArea
    signal clicked(real x, real y)
    signal dragEnded(real srcX, real srcY, real destX, real destY)
    signal dragStarted(real srcX, real srcY, real destX, real destY)
    signal scrolled(real delta);

    //    onDragChanged: console.log("dragging changed:", drag)
    Item {
        id: clickedController

        property point initialPos: Qt.point(0, 0)

        signal clicked(real x, real y)

        function pressed(event) {
            initialPos.x = event.x;
            initialPos.y = event.y;
        }
        function released() {
            if (dragController.drag)
                return;
            clicked(initialPos.x, initialPos.y);
        }

        onClicked: function (x, y) {
            dragArea.clicked(x, y);
        }
    }
    Item {
        id: dragController

        property point curPos: Qt.point(0, 0)
        property bool drag: false
        property point initialPos: Qt.point(0, 0)

        signal dragEnded(real srcX, real srcY, real destX, real destY)
        signal dragStarted(real srcX, real srcY, real destX, real destY)

        function pos_changed(event, pressed) {
            if (!pressed)
                return; // How?
            if (drag) {
                update_cur_pos(event);
                return;
            }
            var threshold = 8;
            var diff = Math.abs(initialPos.x - event.x);
            diff = Math.max(diff, Math.abs(initialPos.y - event.y));
            if (diff > threshold) {
                drag = true;
                dragStarted(initialPos.x, initialPos.y, event.x, event.y);
                update_cur_pos(event);
                return;
            }
        }
        function pressed(event) {
            initialPos.x = event.x;
            initialPos.y = event.y;
        }
        function released() {
            if (!drag)
                return;
            drag = false;
            dragEnded(initialPos.x, initialPos.y, curPos.x, curPos.y);
        }
        function update_cur_pos(event) {
            const rect_x = Math.max(Math.min(event.x, dragArea.x + dragArea.width - 1), dragArea.x);
            const rect_y = Math.max(Math.min(event.y, dragArea.y + dragArea.height - 1), dragArea.y);
            curPos.x = rect_x;
            curPos.y = rect_y;
        }

        onDragEnded: function (srcX, srcY, destX, destY) {
            dragArea.dragEnded(srcX, srcY, destX, destY);
        }
        onDragStarted: function (srcX, srcY, destX, destY) {
            dragArea.dragStarted(srcX, srcY, destX, destY);
        }
    }
    MouseArea {
        id: inner
        anchors.fill: parent

        onPositionChanged: function (mouse) {
            dragController.pos_changed(mouse, pressed);
        }
        onPressed: function (mouse) {
            clickedController.pressed(mouse);
            dragController.pressed(mouse);
        }
        onReleased: {
            clickedController.released(); // has to be before dragCon
            dragController.released();
        }
        onWheel: function (wheel) {
            dragArea.scrolled(wheel.angleDelta.y);
        }
    }
}
