import QtQuick 2.0
import QtQuick.Controls 2.12
import LibHerald 1.0
import "../js/utils.mjs" as Utils

Row {
    id: expireInfo
    anchors.right: parent.right
    anchors.rightMargin: CmnCfg.smallMargin
    anchors.top: parent.top
    anchors.topMargin: CmnCfg.smallMargin
    property alias expireTime: expireTime.text
    property bool isShort: messageModelData.expirationTime
                           !== undefined ? ((messageModelData.expirationTime
                                             - messageModelData.insertionTime)
                                            < 300000 ? true : false) : false

    Label {
        id: expireTime
        text: messageModelData.expirationTime
              !== undefined ? Utils.expireTimeShort(
                                  messageModelData.expirationTime,
                                  messageModelData.insertionTime) : ""
        anchors.top: clock.top
        font.family: CmnCfg.chatFont.name
        font.pixelSize: 13
        color: isShort ? CmnCfg.palette.alertColor : CmnCfg.palette.black
    }

    Button {
        id: clock
        icon.source: timerIcon
        icon.height: 16
        icon.width: 16
        icon.color: isShort ? CmnCfg.palette.alertColor : "grey"
        padding: 0
        background: Item {}
    }
}
