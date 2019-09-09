import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import QtQuick.Controls.Styles 1.4
import LibHerald 1.0
import "../common" as Common
import "../common/utils.js" as Utils
import "ChatTextAreaUtils.js" as CTUtils

Rectangle {
    id: self
    property string messageText: ""
    //color of the bubble proper
    property color bubbleColor
    // the reply button shown on hover
    property alias replyButton: replyButton
    // a mouse area to handle hover events
    property alias chatBubbleHitbox: chatBubbleHitbox
    // the width the text sits at without wrapping
    readonly property int naturalTextWidth: messageMetrics.width
    // a component to use if there is additional content to spawn inside the chat bubble
    property string additionalContent: ""
    // the args to pass into the content spawner
    property var contentArgs
    // an extra margin is needed if there is additionalContent
    readonly property real marginCount: if (additionalContent === "") { 1.5 } else { 2 }

    color: bubbleColor
    radius: QmlCfg.radius

   // Component.onCompleted: { CTUtils.maybeSpawn(additionalContent, contentArgs, attachmentLoader) }
    implicitWidth: bubble.implicitWidth
    implicitHeight: bubble.implicitHeight
    TextMetrics {
        id: messageMetrics
        text: messageText
    }


    MouseArea {
        propagateComposedEvents: true
        id: chatBubbleHitbox
        hoverEnabled: true
        width: parent.width + 50

        onEntered: {
            replyButton.visible =! replyButton.visible
        }

        onExited: {
            replyButton.visible =! replyButton.visible
        }

      anchors {
        left: if(!outbound) parent.left
        right: if(outbound) parent.right
        bottom: parent.bottom
        top: parent.top
      }
      // Emoji button proper
      Button {
         onClicked: {
              CTUtils.activateReplyPopup();
              print("kaavya! put some business logic here.")
          }
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

    ColumnLayout {
        id: bubble
       // width:  Math.max(bubbleText.width, timeStamp.width, attachmentLoader.prefferredWidth) +  QmlCfg.margin
       // height: bubbleText.height + attachmentLoader.height  + timeStamp.height + marginCount * QmlCfg.margin



        TextEdit {
            id: bubbleText
            text: messageText

            Layout.preferredWidth: Math.min(2*chatPane.width / 3, messageMetrics.width) + QmlCfg.margin
            Layout.alignment: Qt.AlignLeft
            Layout.margins: QmlCfg.margin / 2
            Layout.bottomMargin: 0
            wrapMode: TextEdit.Wrap
            selectByMouse: true
            selectByKeyboard: true
            readOnly: true
        }

        // This is a pseudo loader! it is the parent for
        // objects spawned if there is an additional
        // content flag
        Item {
            property int prefferredWidth: 0
            id: attachmentLoader
        }

        Label {
            id: timeStamp
            color: QmlCfg.palette.secondaryTextColor
            text: Utils.friendly_timestamp(epoch_timestamp_ms)
            font.pointSize: 10
            Layout.margins: QmlCfg.margin / 2
            Layout.topMargin: 0
        }
    }
}
