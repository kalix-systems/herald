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

    onPressAndHold: {
        chatList.closeDropdown()
        cb.isSelected = true
        dropdown.activate()
    }

    onPressed: {
        cb.isSelected = false
        dropdown.deactivate()
    }
}
