import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import QtQuick.Controls.Styles 1.4
import LibHerald 1.0
import "../common" as Common
import "../common/utils.js" as Utils

Column {
    property string messageText: ""
    property bool showAvatar: false
    property color bubbleColor
    property int bubbleWidth: 0
    property int bubbleHeight: 0

    // the width the text sits at without wrapping
    property int naturalTextWidth: messageMetrics.width

    // the items which potentially constraint width
    property var widthConstraintArray: [bubbleText.width, timeStamp.width, attachmentLoader.width]

    property Component additionalContent

    TextMetrics {
        id: messageMetrics
        text: messageText
    }

    Rectangle {
        id: bubble
        color: bubbleColor
        radius: QmlCfg.radius
        width: Math.max(...widthConstraintArray) + QmlCfg.margin
        height: bubbleText.height + attachmentLoader.height  + timeStamp.height + QmlCfg.margin

        TextEdit {
            id: bubbleText
            text: messageText

            width: Math.min(2*chatPane.width / 3, messageMetrics.width) + QmlCfg.margin

            anchors {
                margins: QmlCfg.margin / 2
                bottom: timeStamp.top
                left: bubble.left
            }

            wrapMode: TextEdit.Wrap
            selectByMouse: true
            selectByKeyboard: true
            readOnly: true
        }


        Loader {
            active: additionalContent
            id: attachmentLoader
            sourceComponent: additionalContent
            anchors {
                margins: QmlCfg.margin / 2
                horizontalCenter: bubble.horizontalCenter
                bottom: bubbleText.top
            }
        }

        Label {
            id: timeStamp
            color: QmlCfg.palette.secondaryTextColor
            text: Utils.friendly_timestamp(epoch_timestamp_ms)
            font.pointSize: 10
            anchors {
                margins: QmlCfg.margin / 2
                bottom: bubble.bottom
                left: bubble.left
            }
        }
    }
}
