import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import QtQuick 2.12
import LibHerald 1.0
import "../../Common"

ToolBar {
    id: conversationViewHeader

    property string title
    clip: true
    height: QmlCfg.toolbarHeight

    background: Rectangle {
        color: QmlCfg.palette.secondaryColor
    }

    RowLayout {
        anchors {
            fill: parent
            rightMargin: QmlCfg.margin
            leftMargin: QmlCfg.margin
        }

        IconButton {
            id: drawerButton
            Layout.alignment: Qt.AlignLeft
            imageSource: "qrc:/back-arrow-icon.svg"
            tapCallback: function () {
                appState.state = "contact"
            }
        }

        Row {
            Layout.alignment: Qt.AlignCenter
            Label {
                text: title
                font.pointSize: QmlCfg.chatPreviewSize
                font.family: QmlCfg.chatFont.name
                anchors.verticalCenter: parent.verticalCenter
                color: QmlCfg.palette.iconMatte
            }
        }

        IconButton {
            id: searchButton
            Layout.alignment: Qt.AlignRight
            imageSource: "qrc:/search-icon.svg"
        }
    }
}
