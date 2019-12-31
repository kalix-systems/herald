import QtQuick 2.14
import QtQuick.Controls 2.14
import LibHerald 1.0
import "qrc:/imports/ChatBubble"

MouseArea {
    property ChatBubble cb
    property OptionsDropdown dropdown

    pressAndHoldInterval: 350
    hoverEnabled: true

    anchors.fill: parent
    z: CmnCfg.overlayZ

    onPressAndHold: {
        cb.hoverHighlight = true
        dropdown.activate()
    }
    onReleased: {
        cb.hoverHighlight = false
    }
    onExited: {
        cb.hoverHighlight = false
    }
}
