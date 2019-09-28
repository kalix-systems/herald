import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import LibHerald 1.0
import QtQuick.Dialogs 1.3
import "." as CVUtils
import "../common/utils.mjs" as Utils
import "./ChatTextAreaUtils.mjs" as CTUtils
import "../common" as Common
import "../EmojiKeyboard" as EK

Pane {
    id: chatPane
    padding: 0
    property alias messageBar: messageBar
    property Messages ownedConversation


    /// bar at the top that displays the avatar
    CVUtils.ChatBar {
        id: messageBar
    }

    Common.Divider {
        height: 1
        anchors.bottom: messageBar.bottom
        color: "black"
    }

    ///--- chat view, shows messages
    CVUtils.ConversationWindowForm {
        id: convWindow
        focus: true
        anchors {
            top: messageBar.bottom
            bottom: chatTextArea.top
            left: parent.left
            right: parent.right
        }
        Component.onCompleted: forceActiveFocus()
        Keys.onUpPressed: chatScrollBar.decrease()
        Keys.onDownPressed: chatScrollBar.increase()
        Connections {
            target: ownedConversation
            onRowsInserted: {
                convWindow.contentY = convWindow.contentHeight
            }
        }
    }

    Component{
        id: emojiPickerComp
        EK.EmojiPicker {
              id: emojiPicker
              window: parent.window
              Component.onCompleted: {
                  emojiPicker.send.connect((emoji) =>{
                   CTUtils.appendToTextArea(emoji,chatTextArea.chatText);
                    })
              }
              MouseArea {
                  id: block
                  z: exit.z + 1
                  anchors.fill: parent
                  // just blocks input to exit
              }
        }
    }

    MouseArea {
        id: exit
        anchors.fill: convWindow
        onClicked: {
            emoKeysPopup.active = false
        }
    }

     /// Q: why is this not a popup?
     /// A: We don't actually want to load 1000 emojis
     /// in a repeater everytime we open a chat.
    Loader {
        id: emoKeysPopup
        property var window: convWindow
        active: false
        sourceComponent: emojiPickerComp
        anchors.bottom: chatTextArea.top
        anchors.left: chatTextArea.left
    }

    ///--- Text entry area, for typing
    CVUtils.TextAreaForm {
        id: chatTextArea

        anchors {
            left: parent.left
            right: parent.right
            bottom: parent.bottom
            margins: QmlCfg.margin
        }

        keysProxy: Item {
            Keys.onReturnPressed: CTUtils.enterKeyHandler(
                                      event, chatTextArea.chatText,
                                      // this is actually a text area TODO rename
                                      networkHandle, ownedConversation)
            // TODO: Tab should cycle through a hierarchy of items as far as focus
        }
        emojiButton.onClicked: emoKeysPopup.active = !!!emoKeysPopup.active
        atcButton.onClicked: chatTextArea.attachmentsDialogue.open()
        scrollHeight: Math.min(contentHeight, 100)
    }
}
