import QtQuick 2.13
import QtQuick.Controls 2.14
import LibHerald 1.0
import QtQuick.Layouts 1.12
import QtQuick.Window 2.13
import QtGraphicalEffects 1.0

Popup {
    id: colorWindow
    width: CmnCfg.units.dp(180)
    height: width
    property var colorCallback: function () {}

    padding: 0
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
            model: CmnCfg.palette.avatarColors

            Rectangle {
                id: colorDot
                width: CmnCfg.units.dp(52)
                height: width
                radius: width / 2
                color: modelData

                TapHandler {
                    id: tap
                    onLongPressed: {

                        colorDot.color = Qt.darker(modelData, 1.1)
                    }

                    onTapped: {
                        colorIndex = index
                        colorCallback()

                        colorWindow.close()
                    }
                }
            }
        }
    }
}
