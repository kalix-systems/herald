import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Layouts 1.12
import QtQuick.Window 2.13

Window {
    id: colorWindow
    width: QmlCfg.popupWidth
    height: 250
    maximumHeight: height
    minimumHeight: height
    maximumWidth: width
    minimumWidth: width
    title: "Choose Color"
    property var colorIndex

    GridLayout {
        width: QmlCfg.popupWidth
        height: width
        columns: 3
        rows: 3
        anchors.margins: QmlCfg.smallMargin

        Repeater {
            model: QmlCfg.avatarColors

            Rectangle {
                Layout.fillWidth: true
                Layout.fillHeight: true
                color: modelData
                radius: width / 2
                border.color: QmlCfg.palette.tertiaryColor
                border.width: focus ? 2 : 0

                MouseArea {
                    anchors.fill: parent
                    onClicked: {
                        parent.focus = true
                        colorIndex = index
                    }
                }
            }
        }
    }
}
