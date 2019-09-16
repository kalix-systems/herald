import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Layouts 1.12
import QtQuick.Window 2.13

Window {
    id: colorWindow
    width: 200
    height: 250
    maximumHeight: height
    minimumHeight: height
    maximumWidth: width
    minimumWidth: width
    title: "Choose Color"
    property string chosenColor
    property var colorIndex

    GridLayout {
        width: 200
        height: width
        columns: 3
        rows: 3
        anchors.margins: QmlCfg.margin / 2

        Repeater {
            model: QmlCfg.avatarColors

            Rectangle {
                Layout.fillWidth: true
                Layout.fillHeight: true
                color: modelData
                radius: width * 0.5
                border.color: QmlCfg.palette.tertiaryColor
                border.width: 0

                MouseArea {
                    anchors.fill: parent
                    onClicked: {
                        parent.border.width = 2
                        colorIndex = index
                    }
                }
            }
        }
    }
}
