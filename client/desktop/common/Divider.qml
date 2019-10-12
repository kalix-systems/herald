import QtQuick 2.4
import QtQuick.Controls 2.13

Rectangle {
    // BNOTE: This is a confusing name
    property var anchor
    anchors.bottom: anchor
    width: parent.width
    height: 1
}
