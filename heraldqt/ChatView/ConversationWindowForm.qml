import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import LibHerald 1.0
import QtQuick.Dialogs 1.3
import "." as CVUtils
import "../common/utils.js" as Utils

Flickable {
    property alias chatScrollBar: chatScrollBar
    property alias chatListView: chatListView

    clip: true
    interactive: true
    boundsBehavior: Flickable.StopAtBounds
    contentHeight: textMessageCol.height

    ScrollBar.vertical: ScrollBar {
        id: chatScrollBar
        width: 10
    }

    Column {
        id: textMessageCol
        focus: true
        spacing: QmlCfg.margin
        topPadding: QmlCfg.margin
        anchors {
            right: parent.right
            left:parent.left
        }

        Repeater {
            anchors.fill: parent
            id: chatListView
            model: messageModel
            delegate: Column {
                readonly property bool outbound: author === config.config_id

                anchors {
                    right: if (outbound) { parent.right }
                    rightMargin: chatScrollBar.width + QmlCfg.margin
                    leftMargin: rightMargin
                }

                CVUtils.ChatBubble {
                    messageText: body
                    bubbleColor: if (outbound) {
                                       QmlCfg.palette.tertiaryColor
                                   } else {
                                       QmlCfg.palette.secondaryColor
                                   }
                } //bubble
            } //bubble wrapper
        }// Repeater
    } //singleton Col
} // flickable


/*##^## Designer {
    D{i:0;autoSize:true;height:480;width:640}
}
 ##^##*/
