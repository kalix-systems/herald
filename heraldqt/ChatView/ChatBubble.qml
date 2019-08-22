import QtQuick 2.13
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0

/// Avatar:
/// Obj
Row {
    id : avatarRow
    property string text: ""
Rectangle {
    TextMetrics {
        id: messageMetrics
        text: avatarRow.text
    }
    id: bubble
    color: outbound ? QmlCfg.palette.tertiaryColor : QmlCfg.palette.secondaryColor
    radius: QmlCfg.radius
    width: bubbleText.width + 10
    height: bubbleText.height + 10
    Label {
        property bool tooLong: (messageMetrics.width >= chatPane.width / 2)
        id: bubbleText
        wrapMode: Text.Wrap
        width: tooLong ? chatPane.width / 2 : undefined
        text: messageMetrics.text
        anchors.centerIn: bubble
    }
  }
}
