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

    SystemPalette { id: palette;  }

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
    ScrollView {
        bottomPadding: QmlCfg.margin * 2
        clip: true
        anchors {
            top: messageBar.bottom
            bottom: chatTextAreaScroll.top
            left: parent.left
            right: parent.right
        }

        ///--- scrollbar for chat messages
        ScrollBar.vertical: ScrollBar {
            id: chatScrollBar
            Component.onCompleted: position = 1.0
            height: parent.height
            anchors.right: parent.right
        }

        Column {
            width: chatPane.width
            spacing: QmlCfg.margin

            Repeater {
                anchors.fill: parent
                id: chatListView
                Component.onCompleted: forceActiveFocus()
                model: messageModel

                MouseArea {
                    anchors.fill: parent
                    onClicked: forceActiveFocus()
                }

                Keys.onUpPressed: chatScrollBar.decrease()
                Keys.onDownPressed: chatScrollBar.increase()

                delegate: Column {
                    readonly property bool outbound: author === config.config_id

                    anchors {
                        right: if (outbound) {
                                   return parent.right
                               }
                        rightMargin: chatScrollBar.width * 1.5
                    }

                    CVUtils.ChatBubble {
                        topPadding: if (index === 0) {
                                        return QmlCfg.margin
                                    } else {
                                        return 0
                                    }
                        text: body
                    }
                    Component.onCompleted: chatScrollBar.position = 1.0
                } /// Delegate column
            } /// Repeater
        } /// Column
    } /// ScrollView

    ///--- Text entry area
    CVUtils.TextArea {
        id: chatTextAreaScroll
        parentPage: parent
    }

//    ScrollView {
//        clip: true

//        anchors {
//            bottom: parent.bottom
//            left: parent.left
//        }

//        background: Rectangle {
//            color: QmlCfg.palette.mainColor
//        }
//        height: Math.min(contentHeight, 100)
//        width: chatPane.width - attachmentsButton.width - QmlCfg.margin

//        //highlight border
//        onFocusChanged: {
//            if (focus) {
//                chatText.background.border.width = 2
//            } else {
//                chatText.background.border.width = 0
//            }
//        }

//        TextArea {
//            id: chatText

//            background: Rectangle {
//                color: QmlCfg.palette.secondaryColor
//                border.color: QmlCfg.palette.tertiaryColor
//                border.width: 0

//                anchors {
//                    fill: parent
//                    margins: QmlCfg.margin / 2
//                }
//                radius: QmlCfg.radius
//            }

//            selectByKeyboard: true
//            selectByMouse: true
//            padding: QmlCfg.margin
//            wrapMode: TextArea.WrapAtWordBoundaryOrAnywhere
//            placeholderText: "Send a Message ..."
//            Keys.onReturnPressed: {
//
//            }
//            Keys.onEscapePressed: {
//                chatListView.forceActiveFocus()
//            }
//        } /// Chat entry field
//    } /// scroll area

    states: State {
        name: "visibleview"
        PropertyChanges {
            target: chatPane
            opacity: 100
            enabled: true
        }
    }
}
