import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import QtQuick 2.12
import LibHerald 1.0
// Includes CVFLoatingButton. ListItem, and Header
import "./Controls"
import "../Common"
import QtGraphicalEffects 1.0
import Qt.labs.platform 1.0
import "qrc:/imports" as Imports
import "qrc:/imports/Entity"
import "qrc:/imports/NewGroupFlow"
import "qrc:/imports/js/utils.mjs" as Utils
import "GroupFlowComponents"

Page {
    id: newGroupView
    height: mainView.height

    background: Rectangle {
        color: CmnCfg.palette.white
    }

    RowLayout {
        anchors.fill: parent
        Row {
            Layout.alignment: Qt.AlignLeft
            Layout.leftMargin: CmnCfg.units.dp(12)
            spacing: CmnCfg.units.dp(16)
            AnimIconButton {
                id: backButton
                color: CmnCfg.palette.iconFill
                imageSource: "qrc:/back-arrow-icon.svg"
                tapCallback: function () {
                    mainView.pop(null)
                }
            }
        }

        Label {
            id: stateLabel
            text: qsTr("New group")
            font {
                pixelSize: CmnCfg.headerFontSize
                family: CmnCfg.labelFont.name
            }
            Layout.alignment: Layout.verticalCenter
            color: CmnCfg.palette.iconFill
        }
    }

    ColumnLayout {
        anchors.left: parent.left
        anchors.right: parent.right
        anchors.top: parent.top
        anchors.topMargin: CmnCfg.units.dp(40)

        GroupImageSelector {
            id: imageSelector
            // TODO uncomment and test once we display group avatar photos
            // in the mobile UI to make sure this is working; also check
            // commented out section of TapHandler function below
            //imageSource: groupPane.profPicSource
            color: CmnCfg.palette.black
            iconColor: CmnCfg.palette.lightGrey

            Layout.alignment: Qt.AlignTop | Qt.AlignHCenter
        }

        Imports.BorderedTextField {
            id: titleText
            placeholderText: qsTr("Group title")
            color: CmnCfg.palette.black
            borderColor: CmnCfg.palette.black
            Layout.fillWidth: parent
            Layout.leftMargin: CmnCfg.megaMargin
            Layout.rightMargin: CmnCfg.megaMargin
        }

        //TODO: This doesn't do anything yet
        CheckBox {
            topPadding: CmnCfg.units.dp(12)
            text: qsTr("Enable channels")
            font.family: CmnCfg.chatFont.name
            checked: false
            indicator.width: CmnCfg.units.dp(18)
            indicator.height: CmnCfg.units.dp(18)
            Layout.leftMargin: CmnCfg.megaMargin
        }

        Rectangle {
            //anchors.top: topRect.bottom
            id: bigDivider
            height: 1
            width: parent.width
            color: CmnCfg.palette.black
        }

        ContactsSearchComponent {
            id: groupSelectText

            Layout.alignment: Qt.AlignHCenter
        }

        Button {
            Layout.preferredWidth: CmnCfg.units.dp(80)
            Layout.preferredHeight: CmnCfg.units.dp(40)
            Layout.alignment: Qt.AlignRight
            Layout.rightMargin: CmnCfg.megaMargin

            background: Rectangle {
                anchors.fill: parent
                color: CmnCfg.palette.offBlack
            }

            Text {
                text: qsTr("CREATE")
                anchors.centerIn: parent
                color: CmnCfg.palette.white
                font.family: CmnCfg.labelFont.name
            }
            TapHandler {
                onTapped: {
                    if (titleText.text === "") {
                        Herald.conversationBuilder.setTitle(
                                    qsTr("Untitled Group"))
                    } else {
                        Herald.conversationBuilder.setTitle(titleText.text)
                    }

                    if (imageSelector.imageSource !== "") {
                        var parsed = JSON.parse(Herald.utils.imageDimensions(
                                                    imageSelector.imageSource))

                        const picture = {
                            "width": Math.round(parsed.width),
                            "height": Math.round(parsed.height),
                            "x": 0,
                            "y": 0,
                            "path": imageSelector.imageSource
                        }

                        Herald.conversationBuilder.setProfilePicture(
                                    JSON.stringify(picture))
                    }

                    Herald.conversationBuilder.finalize()
                    mainView.pop()
                }
            }
        }
    }

    Component.onCompleted: Herald.usersSearch.refresh()
    Component.onDestruction: Herald.conversationBuilder.clear()
}
