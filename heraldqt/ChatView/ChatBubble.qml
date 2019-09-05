import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import LibHerald 1.0
import "ChatBubble.js" as JS
import "../common" as Common


Row {
    id: avatarRow

//    Common.Avatar {
//        size: 50
//    }

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
        width: Math.max( bubbleText.width, timeStamp.width) + QmlCfg.margin
        height: bubbleText.height + timeStamp.height + QmlCfg.margin
        TextEdit {
            id: bubbleText
            selectByMouse: true
            readOnly: true
            text: messageMetrics.text
            wrapMode: Text.Wrap
            width: JS.calculate_width(chatPane.width, messageMetrics.width)
            anchors{
                margins: QmlCfg.margin/2
                top: bubble.top
                left: bubble.left
                topMargin: QmlCfg.margin/2
            }
        }
        Label {
            id: timeStamp
            color: QmlCfg.palette.secondaryTextColor
            text: qsTr("4 hr ago")
            anchors{
                margins: QmlCfg.margin/2
                bottom: bubble.bottom
                left: bubble.left
                bottomMargin: QmlCfg.margin/2
            }
        }
    }
}
