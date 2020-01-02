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
import "qrc:/imports/ChatBubble" as CB

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

    property string replyText: ""
    property string replyName: ""
    property bool owned: replyUid === Herald.config.configId
    property string replyUid

    property var replyId

    color: CmnCfg.palette.white
    clip: true
    height: containerCol.height

    Imports.IconButton {
        id: attachmentsButton
        anchors.right: parent.right
        anchors.rightMargin: CmnCfg.defaultMargin
        anchors.bottom: parent.bottom
        bottomPadding: CmnCfg.smallMargin * 0.5
        source: "qrc:/attach-icon.svg"
    }

    Imports.IconButton {
        id: emojiButton
        anchors.left: parent.left
        anchors.leftMargin: CmnCfg.defaultMargin
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

        Column {
            width: textWrapperRect.width
            spacing: CmnCfg.smallMargin
            anchors.horizontalCenter: parent.horizontalCenter

            Loader {
                id: replyLoader
                property string opName: replyName
                property string opText: replyText
                active: ownedConversation.builder.isReply
                height: item ? item.height : 0
                sourceComponent: ownedConversation.builder.opAuxContent.length
                                 === 0 ? replyComp : auxComp

                Component {
                    id: replyComp
                    CB.ComposeReplyComponent {
                        builderData: ownedConversation.builder
                    }
                }
                Component {
                    id: auxComp
                    CB.ComposeReplyAuxComponent {
                        builderData: ownedConversation.builder
                    }
                }

                anchors.left: parent.left
                anchors.right: parent.right
                anchors.leftMargin: CmnCfg.smallMargin
                anchors.rightMargin: CmnCfg.smallMargin
            }

            Loader {
                id: attachmentLoader
                active: ownedConversation.builder.hasMediaAttachment
                height: item ? item.height : 0
                width: scrollView.width
                sourceComponent: ImageAttachmentsComponent {}
                anchors.left: parent.left
                anchors.right: parent.right
                anchors.leftMargin: CmnCfg.smallMargin
                anchors.rightMargin: CmnCfg.smallMargin
            }

            Loader {
                id: fileLoader
                active: ownedConversation.builder.hasDocAttachment
                height: item ? item.height : 0
                sourceComponent: FileAttachmentsComponent {}
                anchors.left: parent.left
                anchors.right: parent.right
                anchors.leftMargin: CmnCfg.smallMargin
                anchors.rightMargin: CmnCfg.smallMargin
            }
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

                //TODO: use system palette.
                bottomPadding: CmnCfg.smallMargin * 0.5
                selectionColor: CmnCfg.palette.highlightColor
                color: CmnCfg.palette.black
                selectByMouse: true
                wrapMode: TextArea.WrapAtWordBoundaryOrAnywhere
                placeholderText: qsTr("Message") + " " + conversationItem.title

                Keys.forwardTo: keysProxy
                Keys.onEscapePressed: focus = false
                onEditingFinished: convWindow.focus = true

                // transfer focus to the compose field
                Connections {
                    target: ownedConversation.builder
                    onOpIdChanged: chatText.forceActiveFocus()
                }
            }
        }
    }

    FileDialog {
        id: attachmentsDialogue
        folder: shortcuts.home
        selectMultiple: true
        onSelectionAccepted: {
            if (fileUrl != "") {
                ownedConversation.builder.addAttachment(fileUrl)
            } else {
                for (var i = 0; i < fileUrls.length; i++) {
                    ownedConversation.builder.addAttachment(fileUrls[i])
                }
            }
        }
    }
}
