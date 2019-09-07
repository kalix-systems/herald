import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import LibHerald 1.0
import QtQuick.Dialogs 1.3
import "." as CVUtils
import "../common/utils.js" as Utils

Flickable {
    clip: true
    interactive: true
    boundsBehavior: Flickable.StopAtBounds
    contentHeight: items.height
    anchors {
        top: messageBar.bottom
        bottom: chatTextAreaScroll.top
        left: parent.left
        right: parent.right
    }
    
    ///--- scrollbar for chat messages
    ScrollBar.vertical: ScrollBar {
        id: chatScrollBar
        width: 10
        anchors.right: parent.right
        anchors.top: parent.top
        anchors.bottom: parent.bottom
    }
    
    Column {
        id: items
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
            } /// Delegate column
        } /// Repeater
    } /// Column
}
