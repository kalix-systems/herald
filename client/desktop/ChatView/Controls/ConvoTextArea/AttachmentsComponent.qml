import QtQuick 2.13
import QtQuick.Controls 2.13
import QtGraphicalEffects 1.13
import QtQuick.Layouts 1.12
import "qrc:/common" as Common
import LibHerald 1.0

ScrollView {
    width: parent.width
    height: wrapperRow.height
    // ScrollBar.horizontal.policy: ScrollBar.AlwaysOn
    ScrollBar.vertical.policy: ScrollBar.AlwaysOff

    Row {
        id: wrapperRow
        height: 100
        Layout.margins: 10
        width: parent.width
        spacing: 5
        Repeater {
            id: imageRepeater
            model: builder
            delegate: Rectangle {
                height: 100
                width: 100
                border.color: image.focus ? "light blue" : "black"
                border.width: image.focus ? 2 : 1
                radius: CmnCfg.radius
                clip: true
                Image {
                    id: image
                    anchors.fill: parent
                    anchors.margins: CmnCfg.smallMargin
                    source: "file:" + attachmentPath
                    fillMode: Image.PreserveAspectCrop
                    asynchronous: true

                    MouseArea {
                        anchors.fill: parent
                        onClicked: {
                            parent.focus = !parent.focus
                        }
                    }

                    Button {
                        anchors.top: parent.top
                        anchors.right: parent.right
                        anchors.margins: CmnCfg.smallMargin
                        background: Rectangle {
                            color: "transparent"
                            width: x.width
                            height: x.height
                        }

                        Image {
                            id: x
                            source: "qrc:/x-icon.svg"
                            anchors.centerIn: parent
                            sourceSize: Qt.size(25, 25)
                        }
                        onClicked: {
                            builder.removeAttachmentByIndex(index)
                        }
                    }
                }
            }
        }
    }
    bottomPadding: 5
}
