import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import QtQuick.Dialogs 1.3
import LibHerald 1.0
import "../../../common" as CMN
import "qrc:/imports"

ColumnLayout {

    RowLayout {
        Layout.fillWidth: true
        StandardLabel {
            text: qsTr("Theme")
            color: "black"
            Layout.leftMargin: CmnCfg.margin
            font.pixelSize: CmnCfg.chatTextSize
        }

        Item {
            Layout.fillWidth: true
        }

        ConfRadio {
            Layout.alignment: Qt.AlignRight
            text: qsTr("Dark")
        }

        ConfRadio {
            Layout.alignment: Qt.AlignRight
            text: qsTr("Light")
            checked: true
        }
    }

    Rectangle {
        color: CmnCfg.palette.darkGrey
        height: 1
        Layout.fillWidth: true
    }

    RowLayout {
        Layout.leftMargin: CmnCfg.margin

        Button {
            text: qsTr("Change display name")
            onClicked: submissionCol.visible = true
            Keys.onEscapePressed: submissionCol.visible = false
        }

        StandardLabel {
            color: "black"
            text: qsTr("Current display name: ") + (Herald.config.name)
            Layout.leftMargin: CmnCfg.margin
            font.pixelSize: CmnCfg.chatTextSize
        }
    }

    ColumnLayout {
        id: submissionCol
        Layout.leftMargin: CmnCfg.margin
        visible: false

        TextArea {
            id: displayNameArea
            placeholderText: qsTr("Enter New Display Name...")
            background: Rectangle {
                color: CmnCfg.palette.lightGrey
            }
        }

        Button {
            text: qsTr("Submit")
            onClicked: {
                submissionCol.visible = false
                Herald.config.name = displayNameArea.text
            }
        }
    }

    Rectangle {
        color: CmnCfg.palette.darkGrey
        height: 1
        Layout.fillWidth: true
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
                "path": fileUrl
            }

            Herald.config.setProfilePicture(JSON.stringify(picture))
            //            imageCrop.imageWidth = parsed.width
            //            imageCrop.imageHeight = parsed.height
            //            imageCrop.imageSource = fileUrl
            //            imageCrop.show()
        }
    }

    RowLayout {
        Layout.leftMargin: CmnCfg.margin
        Button {
            text: qsTr("Select profile picture")
            onClicked: cfgPfp.open()
        }
    }

    Rectangle {
        color: CmnCfg.palette.darkGrey
        height: 1
        Layout.fillWidth: true
    }
}
