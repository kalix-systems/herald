import QtQuick 2.13
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0

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
            property bool tooLong: (messageMetrics.width >= chatPane.width / 2)
            id: bubbleText
            text: messageMetrics.text
            wrapMode: Text.Wrap

            width: if (tooLong) {
                       chatPane.width / 2
                   } else {
                       undefined
                   }
            anchors.centerIn: bubble
        }
    }
}
