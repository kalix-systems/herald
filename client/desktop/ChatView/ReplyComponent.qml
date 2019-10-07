import QtQuick 2.13
import LibHerald 1.0
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12

// Reveiw Key
// OS Dependent: OSD
// Global State: GS
// Just Hacky: JH
// Type Script: TS
// Needs polish badly: NPB
// Factor Component: FC
// FS: Fix scoping

//NPB: just looks kind bad
Rectangle {
    property color startColor: "light blue"
    property string opText: "Optimatric"
    radius: QmlCfg.radius
    color: startColor
    width: parent.width
    height: textCol.height

    Column {
        id: textCol
        anchors.margins: QmlCfg.margin
        Text {
            text: opText
        }
    }
    Rectangle {
        color: startColor
        height: QmlCfg.margin
        anchors.top: parent.bottom
    }
}
