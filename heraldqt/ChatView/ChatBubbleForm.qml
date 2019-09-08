import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import QtQuick.Controls.Styles 1.4
import LibHerald 1.0
import "../common" as Common
import "../common/utils.js" as Utils

Row {
    property string messageText: ""
    property bool showAvatar: false
    property color bubbleColor
    property int bubbleWidth: 0
    property int bubbleHeight: 0

    property alias replyButton: replyButton
    property alias chatBubbleHitbox: chatBubbleHitbox
    // the width the text sits at without wrapping
    readonly property int naturalTextWidth: messageMetrics.width

    // the items which potentially constraint width
    property var widthConstraintArray: [bubbleText.width, timeStamp.width, attachmentLoader.width]

    property var additionalContent
    readonly property real marginCount: if (additionalContent === undefined) { 1.5 } else { 2 }


    TextMetrics {
        id: messageMetrics
        text: messageText
    }

    Rectangle {

        MouseArea {
            propagateComposedEvents: true
            id: chatBubbleHitbox
            hoverEnabled: true
            width: parent.width + 50
          anchors {
            left: if(!outbound) parent.left
            right: if(outbound) parent.right
            bottom: parent.bottom
            top: parent.top
          }
          // Emoji button proper
          Button {
              visible: false
              id: replyButton
              anchors.margins: QmlCfg.margin
              anchors.verticalCenter: chatBubbleHitbox.verticalCenter
              height: 25
              width: height
              background: Image {
                  source: "qrc:///icons/reply.png"
                  height: width
                  scale: 0.9
                  mipmap: true
              }
              z: 10
          }
        }





        id: bubble
        color: bubbleColor
        radius: QmlCfg.radius
        width: Math.max(...widthConstraintArray) + QmlCfg.margin
        height: bubbleText.height + attachmentLoader.height  + timeStamp.height + marginCount * QmlCfg.margin

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
            active: additionalContent !== undefined
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
