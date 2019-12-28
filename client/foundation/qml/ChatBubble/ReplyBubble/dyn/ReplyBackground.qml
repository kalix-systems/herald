import QtQuick 2.14
import QtQuick.Controls 2.12
import LibHerald 1.0

Rectangle {
    property alias clickEnabled: mouseArea.enabled
    color: CmnCfg.palette.lightGrey
    anchors.fill: parent
    border.color: CmnCfg.palette.black
    border.width: 1
    ReplyMouseArea {
        id: mouseArea
    }
}
