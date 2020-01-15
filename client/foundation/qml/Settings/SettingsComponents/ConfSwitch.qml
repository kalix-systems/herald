import QtQuick 2.13
import QtQuick.Controls 2.13
import QtGraphicalEffects 1.13
import LibHerald 1.0

// TODO move this to foundation
Switch {
    id: control

    padding: 0
    spacing: 0
    indicator: Rectangle {
        implicitWidth: 32

        implicitHeight: 12
        radius: 13
        x: control.leftPadding
        y: parent.height / 2 - height / 2
        color: CmnCfg.palette.darkGrey
        Rectangle {
            id: ctrl
            x: control.checked ? parent.width - width : 0
            width: 18
            height: 18
            radius: width
            anchors.verticalCenter: parent.verticalCenter
            color: control.checked ? CmnCfg.palette.offBlack : CmnCfg.palette.white
            Behavior on x {
                NumberAnimation {
                    duration: 100
                }
            }
            Behavior on color {
                PropertyAnimation {
                    duration: 100
                }
            }
        }
        DropShadow {
            color: "#000000"
            radius: 4
            samples: 9
            anchors.fill: ctrl
            source: ctrl
        }
    }
}
