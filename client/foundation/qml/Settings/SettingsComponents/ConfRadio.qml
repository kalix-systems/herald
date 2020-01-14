import QtQuick.Controls 2.13
import QtQuick 2.13
import LibHerald 1.0

// TODO move this to foundation
RadioButton {
    id: control
    padding: 0
    indicator: Rectangle {
        id: indic
        implicitWidth: 19
        implicitHeight: 19
        radius: 18
        border.color: CmnCfg.palette.black
        border.width: 2
        Rectangle {
            radius: 18
            color: CmnCfg.palette.offBlack
            anchors.fill: parent
            anchors.margins: control.checked ? 5 : 10
            Behavior on anchors.margins {
                NumberAnimation {
                    duration: 200
                    easing.type: Easing.OutCirc
                }
            }
        }
    }
    contentItem: Text {
        text: control.text
        font: control.font
        anchors.verticalCenter: indic.verticalCenter
        anchors.verticalCenterOffset: 2
        leftPadding: control.indicator.width + control.spacing
    }
}
