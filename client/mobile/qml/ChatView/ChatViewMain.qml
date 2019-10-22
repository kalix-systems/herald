import QtQuick 2.13
import QtQuick.Controls 2.12
import LibHerald 1.0
import "./Controls"

Page {
    //swappable message model, set by the appstate
    property Messages ownedMessages
    header: ChatViewHeader {}
    TextArea {
        background: Rectangle {
            color: "grey"
        }

        anchors {
            bottom: parent.bottom
            right: parent.right
            left: parent.left
        }
    }
}
