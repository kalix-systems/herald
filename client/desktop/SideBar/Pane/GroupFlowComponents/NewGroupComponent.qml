import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Layouts 1.12
import "../../../common" as Common
import QtQuick.Dialogs 1.3
import QtMultimedia 5.13
import "qrc:/imports/js/utils.mjs" as Utils
import "qrc:/imports" as Imports

Component {

    // this uses a rectangle and anchors instead of a layout because that manages the
    // spacing behaviour better. (there is no change in layout on resize, anchors more correct)
    Rectangle {
        anchors.fill: parent
        color: CmnCfg.palette.offBlack

        //header includes group title and picture settings
        GroupHeaderComponent {
            id: topRect
        }

        TextArea {
            id: titleText
            anchors.top: topRect.bottom
            leftPadding: 12
            color: CmnCfg.palette.white
            placeholderText: "Group title"
        }

        Rectangle {
            anchors.top: titleText.bottom
            id: divider
            height: 1
            width: parent.width - CmnCfg.largeMargin
            anchors.horizontalCenter: parent.horizontalCenter
            color: CmnCfg.palette.lightGrey
        }

        Rectangle {
            anchors.top: divider.bottom
            anchors.topMargin: 20
            id: bigDivider
            height: 1
            width: parent.width
            color: CmnCfg.palette.lightGrey
        }

        //component for searching contacts to add
        ContactSearchComponent {
            id: groupSelectText
            anchors.top: bigDivider.bottom
            anchors.topMargin: 20
        }

        //create group button
        Imports.ButtonForm {
            anchors.top: groupSelectText.bottom
            anchors.topMargin: CmnCfg.smallMargin / 2
            anchors.right: parent.right
            anchors.rightMargin: CmnCfg.largeMargin / 2

            width: 60
            height: 30

            background: Rectangle {
                anchors.fill: parent
                color: CmnCfg.palette.medGrey
            }

            Text {
                text: qsTr("CREATE")
                anchors.centerIn: parent
                color: CmnCfg.palette.black
                font.family: CmnCfg.labelFont.name
            }
            onClicked: {
                if (titleText.text === "") {
                    Herald.conversationBuilder.setTitle(qsTr("Untitled Group"))
                } else {
                    Herald.conversationBuilder.setTitle(titleText.text)
                }

                if (topRect.profPic !== "") {

                    var parsed = JSON.parse(Herald.utils.imageDimensions(
                                                topRect.profPic))

                    const picture = {
                        "width": Math.round(parsed.width),
                        "height": Math.round(parsed.height),
                        "x": 0,
                        "y": 0,
                        "path": topRect.profPic
                    }

                    Herald.conversationBuilder.setProfilePicture(
                                JSON.stringify(picture))
                }

                Herald.conversationBuilder.finalize()
                sideBarState.state = ""
            }
        }

        Component.onCompleted: Herald.usersSearch.refresh()
        Component.onDestruction: Herald.conversationBuilder.clear()
    }
}
