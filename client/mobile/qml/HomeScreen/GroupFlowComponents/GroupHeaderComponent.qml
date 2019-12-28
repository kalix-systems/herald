import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import QtQuick 2.12
import LibHerald 1.0
// Includes CVFLoatingButton. ListItem, and Header
import "./../Controls"
import "../../Common"
import QtGraphicalEffects 1.0
import Qt.labs.platform 1.0

// TODO: Factor this out into foundation.
// ^^^^ FOR ALL FILES IN THIS DIRECTORY
Column {
    id: topRect
    anchors.top: parent.top
    width: mainView.width
    property alias profPic: groupImageLoader.imageSource
    property alias groupTitle: titleText.text
    spacing: CmnCfg.units.dp(12)
    topPadding: CmnCfg.units.dp(24)

    Rectangle {
        anchors.horizontalCenter: parent.horizontalCenter
        id: cameraSection
        width: CmnCfg.units.dp(42)
        height: width
        color: CmnCfg.palette.black
        Loader {
            id: groupImageLoader
            active: false
            //TODO: this is a rage manuever, please ammend
            z: 100
            property string imageSource
            anchors.fill: parent
            sourceComponent: Image {
                //  source: imageSource
                anchors.fill: parent
                fillMode: Image.PreserveAspectCrop
            }
        }

        AnimIconButton {
            anchors.centerIn: parent
            imageSource: "qrc:/camera-icon.svg"
            color: CmnCfg.palette.iconFill

            tapCallback: function () {
                print("TODO implement group pics")
            }
        }
    }

    Column {
        width: parent.width - CmnCfg.units.dp(56)
        anchors.horizontalCenter: parent.horizontalCenter
        TextArea {
            id: titleText
            placeholderText: qsTr("Group title")
            leftPadding: 0
        }

        Rectangle {
            id: divider
            height: 1
            width: parent.width
            color: "black"
        }
        //TODO: This doesn't do anything yet
        CheckBox {
            topPadding: CmnCfg.units.dp(12)
            text: qsTr("Enable channels")
            font.family: CmnCfg.chatFont.name
            checked: false
            indicator.width: CmnCfg.units.dp(18)
            indicator.height: CmnCfg.units.dp(18)
        }
    }
}
