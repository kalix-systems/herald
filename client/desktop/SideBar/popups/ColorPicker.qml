import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Layouts 1.12
import QtQuick.Window 2.13
import QtGraphicalEffects 1.0

//TODO: Make Color Settings Exist
// UNREACHABLE !
Popup {
    id: colorWindow
    width: 180
    height: width
    property var colorCallback: function () {}

    padding: 0
    // anchors.centerIn: parent
    modal: true
    property int colorIndex: -1
    property int selectedIndex: -1
    background: Rectangle {
        color: CmnCfg.palette.offBlack
    }
    enter: Transition {

        NumberAnimation {
            property: "opacity"
            from: 0.0
            to: 1.0
            duration: 200
            easing.type: Easing.OutQuad
        }
    }
    exit: Transition {

        NumberAnimation {
            id: exitAnimation
            property: "opacity"
            from: 1.0
            to: 0.0
            duration: 200
            easing.type: Easing.OutQuad
        }
    }

    Grid {
        anchors.fill: parent
        columns: 3
        rows: 3
        spacing: CmnCfg.smallMargin
        anchors.margins: CmnCfg.smallMargin
        padding: 0

        Repeater {
            model: CmnCfg.avatarColors

            Rectangle {
                width: 48
                height: width
                color: !mouse.containsPress ? modelData : Qt.darker(modelData,
                                                                    1.1)
                radius: width / 2
                border.width: mouse.containsMouse || mouse.containsPress ? 2 : 0
                border.color: CmnCfg.palette.offBlack

                MouseArea {
                    id: mouse
                    anchors.fill: parent
                    cursorShape: Qt.PointingHandCursor
                    hoverEnabled: true
                    onClicked: {
                        colorIndex = index
                        colorCallback()

                        parent.border.width = 0
                        colorWindow.close()
                    }
                }
            }
        }
    }
}
