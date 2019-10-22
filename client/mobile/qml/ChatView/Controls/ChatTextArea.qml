import QtQuick 2.13
import QtQuick.Layouts 1.12
import QtQuick.Controls 2.13
import LibHerald 1.0
import "../../Common"

RowLayout {
    readonly property real textareaHeight: QmlCfg.units.dp(36)
    TextArea {
        id: cta
        Layout.fillWidth: true
        height: textareaHeight
        font {
            pointSize: QmlCfg.chatPreviewSize
            family: QmlCfg.chatFont.name
        }
        color: QmlCfg.palette.mainTextColor
        background: Rectangle {
            anchors.fill: parent
            color: QmlCfg.palette.secondaryColor
        }
    }

    Loader {
        Layout.alignment: Qt.AlignRight | Qt.AlignBottom
        sourceComponent: cta.text.length > 0 ? sendButton : atcButton
    }

    Component {
        id: atcButton
        IconButton {}
    }

    Component {
        id: sendButton
        Button {
            height: textareaHeight
        }
    }
}
