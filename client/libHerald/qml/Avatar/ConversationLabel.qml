import QtQuick 2.12
import QtQuick.Layouts 1.12
import QtQuick.Controls 2.12
import LibHerald 1.0

Item {
    // the group name or displayName of the conversation
    property string contactName
    // the previous message of the conversation, or the empty string
    property string lastBody
    // the previous latest human readable timestamp, or the empty string
    property string lastTimestamp
    // the value of the latest read receipt according to the ReceiptStatus enum
    property int lastReceipt: 0

    // labeling constants
    anchors.fill: parent

    Label {
        id: uid
        anchors {
            top: parent.top
            right: ts.left
            left: parent.left
        }
        font.bold: true
        font.pointSize: 17
        elide: "ElideRight"
        text: contactName
        color: "white"
    }

    Label {
        id: ts
        anchors {
            bottom: uid.bottom
            right: parent.right
        }
        font.pointSize: 13
        text: lastTimestamp
        color: "white"
    }

    Label {
        id: bodyText
        anchors {
            left: parent.left
            right: receiptImage.right
            bottom: parent.bottom
        }
        font.pointSize: 15
        elide: "ElideRight"
        text: lastBody
        color: "white"
    }

    Image {
        id: receiptImage
        anchors {
            bottom: parent.bottom
            right: parent.right
        }
        // in the future this should be some function call from common
        source: "qrc:/check-icon-white.svg"
        sourceSize: Qt.size(QmlCfg.units.dp(12), QmlCfg.units.dp(12))
    }
}
