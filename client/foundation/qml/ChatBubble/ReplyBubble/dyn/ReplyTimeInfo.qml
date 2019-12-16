import QtQuick 2.14
import QtQuick.Layouts 1.14
import QtQuick.Controls 2.14
import LibHerald 1.0
import "../../../js/utils.mjs" as Utils

/// NOTE: Here be dragons, this depends on dynamic scoping
Row {
    spacing: 2

    Label {
        id: replyTs

        font.pixelSize: 10
        text: messageModelData.replyType === 2 ? Utils.friendlyTimestamp(
                                                     messageModelData.opInsertionTime) : ""
        color: CmnCfg.palette.darkGrey
    }

    Button {
        id: clock
        icon.source: messageModelData.opExpirationTime
                     !== undefined ? "qrc:/countdown-icon-temp.svg" : ""
        icon.height: 16
        icon.width: 16
        icon.color: "grey"
        padding: 0
        background: Item {}
        anchors.verticalCenter: replyTs.verticalCenter
    }
}