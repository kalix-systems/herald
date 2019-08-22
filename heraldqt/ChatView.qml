import QtQuick 2.13
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0
import "ChatView"
Pane {
    id: chatPane
    property var messageModel: Messages {
    }
    padding: 0
    ///--- chat view, shows messages

    ScrollView {
        width: chatPane.width
        height:chatPane.height
        clip: true

    ListView {

        anchors {
            right: parent.right
            bottom: chatTextAreaScroll.top
            top: parent.top
            left: parent.left
            topMargin: 20 /// allow one unit of spacing between ceiling and first message
            bottomMargin: 20 /// allow one unit of spacing between base and final message
        }

        boundsBehavior: Flickable.StopAtBounds
        spacing: 10
        model: messageModel
        ///--- scrollbar for chat messages
        ScrollBar.vertical: ScrollBar {
            id: chatScrollBar
            Component.onCompleted: position = 1.0
        }

        delegate: Column {

            readonly property bool outbound: author === config.id

            anchors {
                right: outbound ? parent.right : undefined
                rightMargin: chatScrollBar.width * 2
            }

             ChatBubble {
                  text: body
             }
        } /// Delegate
    } /// ListView

    }

    ///--- Text entry area
    ScrollView {
        clip: true
        id: chatTextAreaScroll
        anchors {
            right: parent.right
            bottom: parent.bottom
            left: parent.left
        }
        background: Rectangle {
            color: QmlCfg.palette.mainColor
        }
        height: Math.min(contentHeight, 100)
        TextArea {
            background: Rectangle {
                color: QmlCfg.palette.secondaryColor
                anchors.fill: parent
                anchors.margins: 5
                radius: QmlCfg.radius
            }

            padding: 10
            wrapMode: TextEdit.WrapAtWordBoundaryOrAnywhere
            placeholderText: "Send a Message ..."
            Keys.onReturnPressed: {
                if (text.length <= 0)
                    return
                messageModel.send_message(text)
                chatScrollBar.position = 1.0
                clear()
            }
        } /// Chat entry field
    } /// scroll area
}

