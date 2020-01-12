import QtQuick 2.13
import "../../common" as Common
import "qrc:/imports" as Imports
import LibHerald 1.0
import QtQuick.Layouts 1.12
import "../Popups" as Popups
import Qt.labs.platform 1.1
import QtQuick.Dialogs 1.3
import QtQuick.Controls 2.3

MouseArea {
    id: chatBubbleHitbox
    width: childBubble.width
    height: childBubble.height
    property var childBubble
    property var highlightItem: childBubble.highlightItem
    propagateComposedEvents: true
    hoverEnabled: true
    acceptedButtons: Qt.NoButton
    property bool download: childBubble.aux ? false : (childBubble.imageAttach
                                                       || childBubble.docAttach)
    onEntered: {
        childBubble.hoverHighlight = true
        childBubble.expireInfo.visible = false
    }
    onExited: {
        if (emojiMenu.reactPopup.active == true) {
            return childBubble.hoverHighlight = true
        }
        childBubble.hoverHighlight = false
        if (isHead)
            childBubble.expireInfo.visible = true
    }
}
