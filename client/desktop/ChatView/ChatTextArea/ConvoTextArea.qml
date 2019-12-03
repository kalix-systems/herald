import QtQuick 2.4
import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Dialogs 1.3
import LibHerald 1.0
import QtQuick.Layouts 1.13
import QtMultimedia 5.13
import "qrc:/imports" as Imports
import "js/ChatTextAreaUtils.mjs" as CTUtils
import "../../common" as Common

Rectangle {
    id: textWrapperRect
    property var parentPage
    // height of the text area, computed in JS
    property int scrollHeight
    // height of the text content proper
    property alias contentHeight: scrollView.contentHeight
    // object to forward keypresses to.
    property var keysProxy
    // the attatchments button
    property alias atcButton: attachmentsButton
    // the emoji Button
    property alias emojiButton: emojiButton
    // the text area
    property alias chatText: chatText
    // summy file Dialog
    property alias attachmentsDialogue: attachmentsDialogue

    // camera button
    // property alias cameraButton: cameraButton
    property string replyText: ""
    property string replyName: ""
    property bool owned: replyUid === herald.config.configId
    property string replyUid

    property var replyId

    color: CmnCfg.palette.white
    clip: true

    height: containerCol.height

    Imports.ButtonForm {
        id: attachmentsButton
        anchors.right: parent.right
        anchors.bottom: parent.bottom
        bottomPadding: CmnCfg.smallMargin * 0.5
        source: "qrc:/attach-icon.svg"
    }

    Imports.ButtonForm {
        id: emojiButton
        anchors.left: parent.left
        anchors.bottom: parent.bottom
        bottomPadding: CmnCfg.smallMargin * 0.5
        source: "qrc:/emoji-icon.svg"
    }

    // wrapper column so replies load
    Column {
        id: containerCol

        anchors {
            left: emojiButton.right
            right: attachmentsButton.left
            leftMargin: CmnCfg.smallMargin * 0.5
            rightMargin: CmnCfg.smallMargin * 0.5
            bottomMargin: CmnCfg.smallMargin * 0.5
        }

        topPadding: CmnCfg.smallMargin * 0.5
        bottomPadding: CmnCfg.smallMargin * 0.5

        Loader {
            id: replyLoader
            property string opName: replyName
            property string opText: replyText
            active: false
            height: item ? item.height : 0
            sourceComponent: ReplyComponent {
                startColor: CmnCfg.avatarColors[herald.users.colorById(
                                                    replyUid)]
            }
            width: textWrapperRect.width
            anchors.horizontalCenter: parent.horizontalCenter
        }

        Loader {
            id: attachmentLoader
            active: false
            height: item ? item.height : 0
            sourceComponent: AttachmentsComponent {}
            width: scrollView.width
        }

        ScrollView {
            id: scrollView
            height: Math.min(contentHeight, 100)
            width: containerCol.width
            focus: true

            TextArea {
                id: chatText
                background: Rectangle {
                    color: CmnCfg.palette.white
                }
                bottomPadding: CmnCfg.smallMargin * 0.5
                selectionColor: CmnCfg.palette.highlightColor
                color: CmnCfg.palette.black
                selectByMouse: true
                wrapMode: TextArea.WrapAtWordBoundaryOrAnywhere
                placeholderText: "Message " + conversationItem.title

                Keys.forwardTo: keysProxy
                Keys.onEscapePressed: focus = false

                onEditingFinished: convWindow.focus = true
            }
        }
    }

    FileDialog {
        id: attachmentsDialogue
        folder: shortcuts.home
        onSelectionAccepted: {
            ownedConversation.builder.addAttachment(attachmentsDialogue.fileUrl)
        }
    }

    states: [
        State {
            name: "replystate"
            when: ownedConversation.builder.isReply
            PropertyChanges {
                target: replyLoader
                active: true
            }
            PropertyChanges {
                target: scrollView
                focus: true
            }
        },

        State {
            name: "attachmentstate"
            when: ownedConversation.builder.isMediaMessage
            PropertyChanges {
                target: attachmentLoader
                active: true
            }
        },

        State {
            name: "default"
            PropertyChanges {
                target: replyLoader
                active: false
            }

            PropertyChanges {
                target: scrollView
                focus: true
            }
        }
    ]
}
