import QtQuick.Controls 2.14
import QtQuick.Layouts 1.12
import QtQuick 2.14
import LibHerald 1.0
import "qrc:/imports/ChatBubble" as CB
import "." as CVUtils
import "qrc:/imports/js/utils.mjs" as Utils
import "../../SideBar/js/ContactView.mjs" as CUtils
import Qt.labs.platform 1.1
import QtQuick.Dialogs 1.3
import QtGraphicalEffects 1.0
import "../Popups" as Popups

Item {
    anchors.fill: parent

    anchors.centerIn: parent
    property var parentBubble
    ChatBubbleHover {
        id: bubbleHoverHandler
        download: parentBubble.aux ? false : (parentBubble.imageAttach
                                              || parentBubble.docAttach)
        onEntered: {
            parentBubble.hoverHighlight = true
            parentBubble.expireInfo.visible = false
        }
        onExited: {
            if (reactPopup.active == true) {
                return parentBubble.hoverHighlight = true
            }
            parentBubble.hoverHighlight = false
            if (isHead || !parentBubble.sameExpiration)
                parentBubble.expireInfo.visible = true
        }
    }

    Popup {
        id: emojiMenu
        width: reactPopup.width
        height: reactPopup.height

        x: chatListView.width - width
        y: if (bubbleLoader.y - chatListView.contentY > height) {
               return -height
           } else {
               return CmnCfg.largeMargin * 2
           }

        onClosed: {
            reactPopup.active = false
            if (!bubbleHoverHandler.containsMouse)
                parentBubble.hoverHighlight = false
        }
        onOpened: parentBubble.hoverHighlight = true

        Popups.EmojiPopup {
            id: reactPopup
            anchors.centerIn: parent
            isReactPopup: true
            x: chatListView.width - width
            z: CmnCfg.overlayZ
            onActiveChanged: if (!active) {
                                 emojiMenu.close()
                             }
            anchors.margins: CmnCfg.smallMargin
        }
    }
}
