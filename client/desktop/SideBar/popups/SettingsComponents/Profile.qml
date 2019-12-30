import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import QtQuick.Dialogs 1.3
import QtQuick.Shapes 1.13
import LibHerald 1.0
import "../../../common" as CMN
import "qrc:/imports" as Imports
import "qrc:/imports/Entity" as Entity
import "qrc:/imports/js/utils.mjs" as Utils

ColumnLayout {

    RowLayout {
        Layout.fillWidth: true
        Layout.rightMargin: CmnCfg.defaultMargin
        Layout.bottomMargin: CmnCfg.defaultMargin

        Imports.StandardLabel {
            text: qsTr("Username: ")
            color: "black"
            Layout.leftMargin: CmnCfg.defaultMargin
            font.pixelSize: CmnCfg.chatTextSize
        }
        Imports.StandardLabel {
            text: Herald.config.configId
            color: CmnCfg.palette.black
            font.pixelSize: CmnCfg.chatTextSize
            font.weight: Font.Medium
        }
    }

    RowLayout {
        Layout.leftMargin: CmnCfg.defaultMargin
        Layout.rightMargin: CmnCfg.megaMargin

        Rectangle {
            height: 60
            width: height

            Layout.bottomMargin: CmnCfg.smallMargin

            Entity.Avatar {
                pfpPath: Herald.config.profilePicture
                color: CmnCfg.palette.avatarColors[Herald.config.color]
                size: parent.height
                textColor: CmnCfg.palette.iconFill
                initials: Utils.initialize(Herald.config.name)

                MouseArea {
                    anchors.fill: parent
                    cursorShape: Qt.PointingHandCursor
                    onClicked: cfgPfp.open()
                }
            }

            // TODO this is a general-purpose "this image can be changed"
            // selector, factor it out
            Shape {
                id: cornerCarat
                anchors {
                    right: parent.right
                    bottom: parent.bottom
                }
                anchors.fill: parent
                ShapePath {
                    fillColor: CmnCfg.palette.darkGrey
                    strokeColor: "#00000000"
                    startX: cornerCarat.width * .8
                    startY: cornerCarat.height
                    PathLine {
                        x: cornerCarat.width
                        y: cornerCarat.height * .8
                    }
                    PathLine {
                        x: cornerCarat.width
                        y: cornerCarat.height
                    }
                    PathLine {
                        x: cornerCarat.width * .8
                        y: cornerCarat.height
                    }
                }
            }
        }

        Imports.BorderedTextField {
            id: displayName
            text: Herald.config.name
            selectByMouse: true
            selectionColor: CmnCfg.palette.highlightColor
            readOnly: true
            font.family: CmnCfg.chatFont.name
            font.pixelSize: CmnCfg.headerFontSize
            font.weight: Font.Medium
            color: CmnCfg.palette.black
            borderColor: CmnCfg.palette.white

            Layout.alignment: Qt.AlignLeft | Qt.AlignVCenter
            Layout.leftMargin: CmnCfg.megaMargin
            Layout.preferredWidth: displayName.contentWidth
        }

        Imports.IconButton {
            id: displayNameEditButton
            fill: CmnCfg.palette.black
            source: "qrc:/pencil-icon.svg"
            Layout.alignment: Qt.AlignLeft
            Layout.leftMargin: CmnCfg.microMargin

            property bool editing: false

            onClicked: {
                if (editing) {
                    displayNameEditButton.editing = false
                    displayName.readOnly = true
                    displayName.borderColor = CmnCfg.palette.white
                    displayName.Layout.fillWidth = false
                    displayNameEditButton.source = "qrc:/pencil-icon.svg"
                    Herald.config.name = displayName.text
                } else {
                    displayNameEditButton.editing = true
                    displayName.readOnly = false
                    displayName.borderColor = CmnCfg.palette.black
                    displayName.Layout.fillWidth = true
                    displayNameEditButton.source = "qrc:/check-icon.svg"
                }
            }
        }
    }

    FileDialog {
        id: cfgPfp
        property bool pfpValid: true
        folder: shortcuts.desktop
        nameFilters: ["(*.jpg *.png *.jpeg)"]
        onSelectionAccepted: {
            var parsed = JSON.parse(Herald.utils.imageDimensions(fileUrl))

            const picture = {
                "width": Math.round(parsed.width),
                "height": Math.round(parsed.height),
                "x": 0,
                "y": 0,
                "path": Herald.utils.stripUrlPrefix(fileUrl)
            }

            Herald.config.setProfilePicture(JSON.stringify(picture))
            //            imageCrop.imageWidth = parsed.width
            //            imageCrop.imageHeight = parsed.height
            //            imageCrop.imageSource = fileUrl
            //            imageCrop.show()
        }
    }

    Rectangle {
        color: CmnCfg.palette.darkGrey
        height: 1
        Layout.fillWidth: true
    }

    // TODO add location information
}
