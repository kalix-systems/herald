import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import LibHerald 1.0
import "ChatBubble.js" as JS
import "../common/utils.js" as Utils


Row {
    id: avatarRow
    property string text: ""
    property int sendStatus: 0
    spacing: QmlCfg.margin

    Loader {
        id: statusIcon
        sourceComponent: if(sendStatus == AckTypes.Timeout) { iconActual }
        width: Utils.unwrap_or(sourceComponent, { width: -QmlCfg.margin }).width + QmlCfg.margin
        anchors {
            verticalCenter: bubble.verticalCenter
        }
        Component {
            id: iconActual
            Rectangle {
                height: 15
                width: height
                color: "black"
            }
        }
    }

    Rectangle {
        TextMetrics {
            id: messageMetrics
            text: avatarRow.text
        }
        id: bubble
        color: if (outbound) {
                   switch (sendStatus) {
                   case AckTypes.Timeout:
                       "pink"
                       break;
                   default:
                         QmlCfg.palette.tertiaryColor
                   }

               } else {
                   QmlCfg.palette.secondaryColor
               }

        radius: QmlCfg.radius

        Label {
            id: bubbleText
            text: messageMetrics.text
            wrapMode: Text.Wrap

            width: JS.calculate_width(chatPane.width, messageMetrics.width)
            anchors.centerIn: bubble
        }



        width: bubbleText.width +  QmlCfg.margin
        height: bubbleText.height  + QmlCfg.margin

    }

}
