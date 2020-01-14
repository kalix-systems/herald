// TODO get rid of extra imports in this list
import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
//import QtQuick.Dialogs 1.3
//import Qt.labs.platform 1.1
import Qt.labs.settings 1.0
import QtQuick.Shapes 1.13
import LibHerald 1.0
import "../../"
import "../../Entity" as Entity
import "../../js/utils.mjs" as Utils

ColumnLayout {
    RowLayout {
        Layout.fillWidth: true
        Layout.rightMargin: CmnCfg.defaultMargin
        Layout.bottomMargin: CmnCfg.defaultMargin

        StandardLabel {
            text: qsTr("Username: ")
            color: "black"
            Layout.leftMargin: CmnCfg.defaultMargin
            font.family: CmnCfg.chatFont.name
            font.pixelSize: CmnCfg.chatTextSize
        }
        StandardLabel {
            text: Herald.config.configId
            color: CmnCfg.palette.black
            font.family: CmnCfg.chatFont.name
            font.pixelSize: CmnCfg.chatTextSize
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
                color: CmnCfg.palette.avatarColors[Herald.config.configColor]
                size: parent.height
                textColor: CmnCfg.palette.iconFill
                initials: Utils.initialize(Herald.config.name)

                MouseArea {
                    anchors.fill: parent
                    cursorShape: Qt.PointingHandCursor
                    onClicked: {
                        if (!fileDialogLoader.active)
                            return

                        fileDialogLoader.item.open()
                    }
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

                MouseArea {
                    anchors {
                        right: parent.right
                        bottom: parent.bottom
                    }
                    height: cornerCarat.height * 0.2
                    width: cornerCarat.width * 0.2
                    cursorShape: Qt.PointingHandCursor
                    onClicked: {

                        //  cropCallbackArg(Herald.config.profilePicture)
                    }
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

        BorderedTextField {
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

        IconButton {
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

    // TODO FileDialog doesn't work on mobile, pass in something that does
    // RESPONSE: this basically needs to be a loader, or we could stop fetishizing code reuse
    // even when it breaks things everywhere
    Loader {
        id: fileDialogLoader

        sourceComponent: fileDialogComponent
        active: fileDialogComponent !== null
        //  FileDialog {
        //      id: cfgPfp
        //      property bool pfpValid: true
        //      folder: shortcuts.desktop
        //      nameFilters: ["(*.jpg *.png *.jpeg)"]
        //      onSelectionAccepted: {

        //          cropCallbackArg(fileUrl)
        //      }
        //  }
    }

    Rectangle {
        color: CmnCfg.palette.darkGrey
        height: 1
        Layout.fillWidth: true
    }

    // TODO add location information
}
