import QtQuick 2.4
import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Layouts 1.13
import QtMultimedia 5.13
import "qrc:/imports" as Imports
import "js/ChatTextAreaUtils.mjs" as CTUtils
import "../../common" as Common
import "qrc:/imports/ChatBubble" as CB
import QtQuick.Dialogs 1.3

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
    property alias timer: timerMenu

    property var replyId

    color: CmnCfg.palette.white
    clip: true
    height: containerCol.height

    Imports.IconButton {
        id: attachmentsButton
        anchors.right: parent.right
        anchors.rightMargin: CmnCfg.smallMargin
        anchors.bottom: parent.bottom
        bottomPadding: CmnCfg.units.dp(4)
        source: "qrc:/attach-icon.svg"
    }
    Imports.IconButton {

        id: timerButton
        source: timerMenu.chosenTimer
        fill: "transparent"
        anchors.left: parent.left
        anchors.leftMargin: CmnCfg.smallMargin
        anchors.bottom: parent.bottom
        bottomPadding: CmnCfg.units.dp(3)
        topPadding: 1
        onClicked: timerMenu.open()
        tooltipText: "Set single-message expiration"
    }

    Imports.TimerOptionsBuilder {
        id: timerMenu
        conversationItem: chatPage.conversationItem
        builder: ownedConversation.builder
    }

    Imports.IconButton {
        id: emojiButton
        anchors.right: attachmentsButton.left
        anchors.rightMargin: CmnCfg.smallMargin
        anchors.bottom: parent.bottom
        bottomPadding: CmnCfg.units.dp(3)
        source: "qrc:/emoticon-icon.svg"
    }

    // wrapper column so replies load
    Column {
        id: containerCol

        anchors {
            left: timerButton.right
            right: emojiButton.left
            leftMargin: CmnCfg.smallMargin * 2
            rightMargin: CmnCfg.smallMargin
            bottomMargin: CmnCfg.smallMargin * 0.5
        }

        topPadding: CmnCfg.smallMargin * 0.5

        //   bottomPadding: CmnCfg.units.dp(1)
        Column {
            width: textWrapperRect.width
            spacing: CmnCfg.smallMargin
            anchors.horizontalCenter: parent.horizontalCenter
            anchors.horizontalCenterOffset: 10

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

        Flickable {
            id: scrollView
            height: Math.min(Math.max(chatText.contentHeight, chatText.height),
                             CmnCfg.units.dp(100))
            width: containerCol.width
            focus: true
            contentWidth: width
            clip: true
            contentHeight: Math.max(chatText.contentHeight, chatText.height)
            leftMargin: 0

            maximumFlickVelocity: 1200
            flickDeceleration: chatText.height * 10

            ScrollBar.vertical: ScrollBar {}
            boundsBehavior: Flickable.StopAtBounds
            boundsMovement: Flickable.StopAtBounds
            contentY: chatText.cursorRectangle.y
            TextEdit {
                id: chatText
                selectionColor: CmnCfg.palette.highlightColor
                color: CmnCfg.palette.black
                selectByMouse: true
                wrapMode: TextArea.WrapAtWordBoundaryOrAnywhere
                width: containerCol.width
                topPadding: CmnCfg.units.dp(5)
                bottomPadding: CmnCfg.units.dp(5)
                rightPadding: CmnCfg.smallMargin

                font.pixelSize: CmnCfg.chatTextSize
                Text {
                    text: qsTr("Message") + " "
                          + (!Herald.utils.compareByteArray(
                                 conversationItem.conversationId,
                                 Herald.config.ntsConversationId) ? conversationItem.title : qsTr(
                                                                        "Note to Self"))
                    color: CmnCfg.palette.darkGrey
                    opacity: (chatText.text.length === 0) ? 1.0 : 0.0
                    anchors.baseline: parent.baseline
                    width: containerCol.width
                    elide: Text.ElideRight
                    font.pixelSize: CmnCfg.chatTextSize
                    font.family: CmnCfg.chatFont.name
                    font.weight: Font.Light
                    padding: 0
                }

                Keys.forwardTo: keysProxy
                Keys.onEscapePressed: focus = false
                onEditingFinished: convWindow.focus = true

                Common.TextContextMenu {
                    parentText: chatText
                }

                // transfer focus to the compose field
                Connections {
                    target: ownedConversation.builder
                    onOpIdChanged: chatText.forceActiveFocus()
                }

                // sends typing indicators
                onTextChanged: {
                    if (text !== "") {
                        Qt.callLater(function () {
                            ownedConversation.sendTypingIndicator()
                        })
                    }
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
            chatText.forceActiveFocus()
        }
    }
}
