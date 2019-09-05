import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import LibHerald 1.0
import "../common" as Common
import "../common/utils.js" as Utils

Row {
    id: avatarRow

    property string text: ""
    property bool showAvatar: false

    Rectangle {
        TextMetrics {
            id: messageMetrics
            text: avatarRow.text
        }
        id: bubble
        color: if (outbound) {
                   QmlCfg.palette.tertiaryColor
               } else {
                   QmlCfg.palette.secondaryColor
               }

        radius: QmlCfg.radius
        width: Math.max(bubbleText.width, timeStamp.width) + QmlCfg.margin
        height: bubbleText.height + timeStamp.height + QmlCfg.margin
        TextEdit {
            id: bubbleText
            text: messageMetrics.text
            selectByMouse: true
            mouseSelectionMode: TextEdit.SelectCharacters
            readOnly: true
            wrapMode: TextEdit.Wrap
            width: Math.min((chatPane.width / 2), messageMetrics.width) + 10
            anchors {
                margins: QmlCfg.margin / 2
                top: bubble.top
                left: bubble.left
                topMargin: QmlCfg.margin / 2
            }
            Component.onCompleted: {
                bubbleText.set
            }
        }
        Label {
            id: timeStamp
            color: QmlCfg.palette.secondaryTextColor
            text: Utils.friendly_timestamp(epoch_timestamp_ms)
            anchors {
                margins: QmlCfg.margin / 2
                bottom: bubble.bottom
                left: bubble.left
                bottomMargin: QmlCfg.margin / 2
            }
        }
    }
}
