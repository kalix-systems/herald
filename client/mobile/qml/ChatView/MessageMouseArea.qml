import QtQuick 2.14
import QtQuick.Controls 2.14
import LibHerald 1.0
import "qrc:/imports/ChatBubble"

MouseArea {
    property var cb
    property OptionsDropdown dropdown

    pressAndHoldInterval: 350
    hoverEnabled: true
    propagateComposedEvents: true

    anchors.fill: parent
    z: CmnCfg.overlayZ

    // TODO message highlight should persist until options menu is closed
    onPressAndHold: {
        cb.hoverHighlight = true
        dropdown.activate()
        chatList.closeDropdown()
    }

    onPressed: {
        dropdown.deactivate()
    }

    onReleased: {
        cb.hoverHighlight = false
    }

    onExited: {
        cb.hoverHighlight = false
    }
}
