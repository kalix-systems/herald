import QtQuick 2.13
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0
import "ChatBubble.js" as JS

/// Avatar:
/// Obj
Row {
    id: avatarRow
    property string text: ""
    Rectangle {
        TextMetrics {
            id: messageMetrics
            text: avatarRow.text
        }
        id: bubble
        color: if (outbound) {
                   QmlCfg.palette.tertiaryColor
               } else {
                   QmlCfg.palette.secondaryColor
               }
        radius: QmlCfg.radius
        width: bubbleText.width + QmlCfg.margin
        height: bubbleText.height + QmlCfg.margin
        Label {
            id: bubbleText
            text: messageMetrics.text
            wrapMode: Text.Wrap

            width: JS.calculate_width(chatPane.width, messageMetrics.width)
            anchors.centerIn: bubble
        }
    }
}
