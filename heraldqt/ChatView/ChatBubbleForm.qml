import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import LibHerald 1.0
import "../common" as Common
import "../common/utils.mjs" as Utils
import "ChatTextAreaUtils.mjs" as CTUtils

// Reveiw Key
// OS Dependent: OSD
// Global State: GS
// Just Hacky: JH
// Type Script: TS
// Needs polish badly: NPB
// Factor Component: FC
// FS: Fix scoping
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

    color: bubbleColor
    radius: QmlCfg.radius

    // NPB: this flickers a lot, pause on scroll also
    MouseArea {
        propagateComposedEvents: true
        id: chatBubbleHitbox
        hoverEnabled: true
        width: parent.width + 50

        //TS: put this logic in a seperate file
        onEntered: {
            replyButton.visible = !replyButton.visible
        }

        //TS: ""
        onExited: {
            replyButton.visible = !replyButton.visible
        }

        anchors {
            left: if (!outbound)
                      parent.left
            right: if (outbound)
                       parent.right
            bottom: parent.bottom
            top: parent.top
        }
        // Emoji button proper
        // FC: more button reuse
        Button {
            id: replyButton
            onClicked: {
                CTUtils.activateReplyPopup()
                print("kaavya! put some business logic here.")
            }
            visible: false
            anchors.margins: QmlCfg.margin
            anchors.verticalCenter: chatBubbleHitbox.verticalCenter
            height: 25
            width: height
            background: Image {
                //FC: replace all icons with constant sources, save on typo hell // refactors
                source: "qrc:///icons/reply.png"
                height: width
                scale: 0.9
                mipmap: true
            }
            z: 10
        }
    }

    //TS: also a massive anti-pattern
    // NPB find a better generic way to spawn items inside of chat bubbles, states and loaders
    Component.onCompleted: {
        contentArgs.uiContainer = bubbleText
        attachmentLoader.setSource(additionalContent, contentArgs)
    }

    width: bubble.width
    height: bubble.height

    TextMetrics {
        id: messageMetrics
        text: messageText
    }

    Column {
        id: bubble
        padding: QmlCfg.margin / 2

        /// NBP: find a better way to generically load content
        Loader {
            id: attachmentLoader
            source: additionalContent
        }

        TextEdit {
            id: bubbleText
            text: messageText
            //TS: NPB: that extra margin is bad, also this is a recipe for a binding loop
            width: Math.min(2 * chatPane.width / 3,
                            messageMetrics.width) + QmlCfg.margin
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
            /// NPB: all font sizes should be settable, for visual stuff
            font.pointSize: 10
        }
    }
}
