import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import LibHerald 1.0
import QtQuick.Dialogs 1.3
import "ChatView" as CVUtils
import "common/utils.js" as Utils


Pane {
    id: chatPane
    enabled: false
    opacity: 0
    padding: 0
    property alias messageBar: messageBar
    property Messages messageModel: Messages {
    }

    CVUtils.ChatBar {
        id: messageBar
    }

    ///--- border between messageBar and main chat view
    Rectangle {
        height: 1
        color: QmlCfg.palette.secondaryColor
        anchors {
            top: messageBar.bottom
            left: parent.left
            right: parent.right
        }
    }

    ///--- chat view, shows messages
   CVUtils.ConversationWindow {
       id: convWindow
       anchors {
           top: messageBar.bottom
           bottom: chatTextAreaScroll.top
           left: parent.left
           right: parent.right
       }
    } /// ScrollView

    ///--- Text entry area
    CVUtils.TextArea {
        id: chatTextAreaScroll
        anchors {
            left: parent.left
            right: parent.right
            bottom: parent.bottom
            margins: QmlCfg.margin
        }
    }

    states: State {
        name: "visibleview"
        PropertyChanges {
            target: chatPane
            opacity: 100
            enabled: true
        }
    }
}
