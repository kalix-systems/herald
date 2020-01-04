import QtQuick 2.0
import QtQuick.Controls 2.12
import LibHerald 1.0
import "../../../js/utils.mjs" as Utils

Row {
    id: expireInfo
    anchors.right: parent.right
    anchors.rightMargin: CmnCfg.smallMargin
    anchors.top: parent.top
    anchors.topMargin: CmnCfg.smallMargin
    property alias expireTime: expireTime.text

    Connections {
        target: appRoot.globalTimer
        onRefreshTime: {

            clock.icon.source = messageModelData.opExpirationTime
                    !== undefined ? Utils.timerIcon(
                                        messageModelData.opExpirationTime,
                                        messageModelData.opInsertionTime) : ""
        }
    }

    Label {
        id: expireTime
        text: messageModelData.expirationTime
              !== undefined ? Utils.expireTimeShort(
                                  messageModelData.opExpirationTime,
                                  messageModelData.opInsertionTime) : ""
        anchors.top: clock.top
        font.family: CmnCfg.chatFont.name
        font.pixelSize: 13
    }

    Button {
        id: clock
        icon.source: messageModelData.opExpirationTime
                     !== undefined ? Utils.timerIcon(
                                         messageModelData.opExpirationTime,
                                         messageModelData.opInsertionTime) : ""

        icon.height: 16
        icon.width: 16
        icon.color: "grey"
        padding: 0
        background: Item {}
    }
}
