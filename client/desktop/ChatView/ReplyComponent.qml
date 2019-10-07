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
    radius: QmlCfg.radius
    color: startColor
    Rectangle {
        height: QmlCfg.margin
        anchors.top: parent.bottom
    }
}
