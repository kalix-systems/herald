import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Layouts 1.12
import QtQuick.Window 2.13

//TODO: Make Color Settings Exist
// UNREACHABLE !
Window {
    id: colorWindow
    width: CmnCfg.popupWidth
    height: CmnCfg.popupHeight
    maximumHeight: height
    minimumHeight: height
    maximumWidth: width
    minimumWidth: width
    title: qsTr("Choose Color")

    property int colorIndex: -1
    property int selectedIndex: -1

    GridLayout {
        width: CmnCfg.popupWidth
        height: width
        columns: 3
        rows: 3
        anchors.margins: CmnCfg.smallMargin

        Repeater {
            model: CmnCfg.avatarColors

            Rectangle {
                Layout.fillWidth: true
                Layout.fillHeight: true
                color: modelData
                radius: width / 2
                border.color: "lightsteelblue"
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

    Button {
        id: colorSubmissionButton
        text: qsTr("Submit")

        anchors {
            right: parent.right
            bottom: parent.bottom
        }

        onClicked: {
            Herald.users.setColor(gsSelectedIndex, avatarColorPicker.colorIndex)
            avatarColorPicker.close()
        }
    }
}
