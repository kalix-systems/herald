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
import "NewGroupFlow"

Page {
    id: newGroupView
    height: mainView.height
    readonly property Component headerComponent: NewGroupHeader {}
    background: Rectangle {
        color: CmnCfg.palette.white
    }

    ColumnLayout {
        anchors.left: parent.left
        anchors.right: parent.right
        anchors.top: parent.top
        anchors.topMargin: CmnCfg.megaMargin

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
            Layout.topMargin: CmnCfg.defaultMargin
        }

        ContactsSearchComponent {
            id: groupSelectText
            Layout.alignment: Qt.AlignHCenter
        }

        Imports.TextButton {
            text: qsTr("CREATE")

            Layout.preferredWidth: CmnCfg.units.dp(80)
            Layout.preferredHeight: CmnCfg.units.dp(40)
            Layout.alignment: Qt.AlignRight
            Layout.rightMargin: CmnCfg.megaMargin

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
                            "path": Herald.utils.stripUrlPrefix(
                                        imageSelector.imageSource)
                        }

                        Herald.conversationBuilder.setProfilePicture(
                                    JSON.stringify(picture))
                    }

                    Herald.conversationBuilder.finalize()
                    // TODO this should be mainView.replace(<new group convo>)
                    mainView.pop()
                }
            }
        }
    }

    Component.onCompleted: Herald.usersSearch.refresh()
    Component.onDestruction: Herald.conversationBuilder.clear()
}
