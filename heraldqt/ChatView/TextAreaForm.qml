import QtQuick 2.4
import QtQuick.Controls 2.13
import QtQuick.Dialogs 1.3
import LibHerald 1.0
import "ChatTextAreaUtils.js" as CTUtils

// Reveiw Key
// OS Dependent: OSD
// Global State: GS
// Just Hacky: JH
// Type Script: TS
// Needs polish badly: NPB
// RS: Rusts job
// Factor Component: FC
Rectangle {

    property var parentPage
    // height of the text area, computed in JS
    property int scrollHeight
    // height of the text content proper
    property int contentHeight: scrollView.contentHeight
    // object to forward keypresses to.
    property var keysProxy
    // the attatchments button
    property alias atcButton: attachmentsButton
    // the emoji Button
    property alias emojiButton: emojiButton
    // the text area
    property alias chatText: chatText
    // clippy file Dialog
    property alias attacmentsDialog : attachmentsDialogue

    color: QmlCfg.palette.mainColor
    clip: true

    /// NPB : why does this need a margin added ?! put in a column.
    height: scrollHeight + QmlCfg.margin

    // FC: this is a common button pattern
    // attatchments button proper
    Button {
        id: attachmentsButton
        anchors.right: parent.right
        anchors.bottom: parent.bottom
        height: 25
        width: height
        background: Image {
            source: "qrc:///icons/paperclip.png"
            height: width
            scale: 0.9
            mipmap: true
        }
    }

    // FC: this is a common button pattern
    // Emoji button proper
    Button {
        id: emojiButton
        anchors.left: parent.left
        anchors.bottom: parent.bottom
        height: 25
        width: height
        background: Image {
            source: "qrc:///icons/emoji.png"
            height: width
            scale: 0.9
            mipmap: true
        }
    }

    ScrollView {
        id: scrollView
        height: scrollHeight
        focus: true

        anchors {
            left: emojiButton.right
            right: attachmentsButton.left
            bottom: parent.bottom
            // FC: small margin vs large margin, /2 is an anti pattern
            leftMargin: QmlCfg.margin/2
            rightMargin: QmlCfg.margin/2
        }

        TextArea {
            id: chatText
            background: Rectangle {
                color: QmlCfg.palette.secondaryColor
                anchors {
                    fill: parent
                    horizontalCenter: parent.horizontalCenter
                    verticalCenter: parent.verticalCenter
                }
                radius: QmlCfg.radius
            }
            selectByMouse: true
            wrapMode: TextArea.WrapAtWordBoundaryOrAnywhere
            placeholderText: "Send a Message ..."
            Keys.forwardTo: keysProxy
        }
    }

    FileDialog {
         id: attachmentsDialogue
         folder: shortcuts.home
         onSelectionAccepted: {
             print("todo: attachments api")
         }
     }
   }
