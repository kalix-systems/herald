import QtQuick 2.4
import QtQuick.Controls 2.13
import LibHerald 1.0

Button {
    id: button
    property bool light: false

    property color bkgrndColor: light ? CmnCfg.palette.lightGrey : CmnCfg.palette.offBlack
    property color textColor: light ? CmnCfg.palette.black : CmnCfg.palette.white
    contentItem: Text {
        text: button.text
        font: CmnCfg.defaultFont
        color: textColor
        horizontalAlignment: Text.AlignHCenter
        verticalAlignment: Text.AlignVCenter
    }

    background: Rectangle {
        implicitHeight: 30
        implicitWidth: 60
        opacity: enabled ? 1 : 0.5
        color: bkgrndColor
    }

    MouseArea {
        anchors.fill: parent
        hoverEnabled: true
        propagateComposedEvents: true
        acceptedButtons: Qt.NoButton
        cursorShape: Qt.PointingHandCursor
    }
}
