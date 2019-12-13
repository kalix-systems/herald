import QtQuick 2.13
import QtQuick.Controls 2.13
import QtGraphicalEffects 1.13
import QtQuick.Layouts 1.12
import "qrc:/common" as Common
import LibHerald 1.0
import "qrc:/imports" as Imports

ScrollView {
    width: parent.width
    height: wrapperRow.height
    ScrollBar.horizontal: ScrollBar {}

    Row {
        id: wrapperRow
        height: 100
        Layout.margins: CmnCfg.mediumMargin
        width: parent.width
        spacing: 5
        Repeater {
            id: imageRepeater
            model: ownedConversation.builder.mediaAttachments
            delegate: Rectangle {
                height: 100
                width: 100
                clip: true
                Image {
                    id: image
                    anchors.fill: parent
                    // anchors.margins: CmnCfg.smallMargin
                    source: "file:" + mediaAttachmentPath
                    fillMode: Image.PreserveAspectCrop
                    asynchronous: true

                    MouseArea {
                        anchors.fill: parent
                        onClicked: {
                            parent.focus = !parent.focus
                        }
                    }

                    ColorOverlay {
                        id: overlay
                        anchors.fill: parent
                        source: parent
                        visible: imageHover.containsMouse
                        color: CmnCfg.palette.black
                        opacity: 0.5
                        smooth: true
                    }

                    Imports.ButtonForm {
                        id: clearPhoto
                        source: "qrc:/x-icon.svg"
                        anchors.centerIn: parent
                        visible: imageHover.containsMouse
                        onClicked: ownedConversation.builder.removeMedia(index)
                        fill: CmnCfg.palette.white
                        opacity: 1.0
                        hoverEnabled: true
                    }

                    MouseArea {
                        anchors.fill: parent
                        hoverEnabled: true
                        id: imageHover
                        onClicked: mouse.accepted = false
                        onReleased: mouse.accepted = false
                        onPressed: mouse.accepted = false
                        cursorShape: Qt.PointingHandCursor
                    }
                }
            }
        }
    }
}
