import QtQuick.Controls 2.14
import QtQuick 2.14
import LibHerald 1.0
import Qt.labs.platform 1.1
import "../Popups" as Popups

Popup {
    id: emojiMenu
    width: reactPopup.width
    height: reactPopup.height
    property alias reactPopup: react

    x: chatListView.width - width
    y: if (bubbleLoader.y - chatListView.contentY > height) {
           return -height
       } else {
           return CmnCfg.largeMargin * 2
       }

    onClosed: {
        reactPopup.active = false
        if (!bubbleActual.hitbox.containsMouse)
            bubbleActual.hoverHighlight = false
    }
    onOpened: bubbleActual.hoverHighlight = true

    Popups.EmojiPopup {
        id: react
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
