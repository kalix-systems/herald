import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import LibHerald 1.0
import "../common" as Common
import "../common/utils.mjs" as Utils
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

    color: bubbleColor
    radius: QmlCfg.radius


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

   Component.onCompleted: { contentArgs.uiContainer =  bubbleText; attachmentLoader.setSource(additionalContent, contentArgs) }

    implicitWidth: bubble.implicitWidth
    implicitHeight: bubble.implicitHeight

    TextMetrics {
        id: messageMetrics
        text: messageText
    }

    Column {
        id: bubble
        padding: QmlCfg.margin / 2

        Loader {
             id: attachmentLoader
             source: additionalContent
        }


        TextEdit {
            id: bubbleText
            text: messageText
            width: Math.min(2*chatPane.width / 3, messageMetrics.width) + QmlCfg.margin
            Layout.alignment: Qt.AlignLeft
            wrapMode: TextEdit.Wrap
            selectByMouse: true
            selectByKeyboard: true
            readOnly: true
        }

        Label {
            id: timeStamp
            color: QmlCfg.palette.secondaryTextColor
            text: Utils.friendlyTimestamp(epoch_timestamp_ms)
            font.pointSize: 10
        }
    }
}



/*##^## Designer {
    D{i:0;autoSize:true;height:480;width:640}
}
 ##^##*/
