import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Layouts 1.12
import "../../common" as Common
import "../popups/js/NewContactDialogue.mjs" as JS
import "../../SideBar" as SBUtils

Component {
    id: groupSelectComponent

    Rectangle {
        height: groupFlow.height
        width: parent.width
        id: wrapperRect
        color: CmnCfg.palette.mainColor
        Common.ButtonForm {
            id: backbutton
            source: "qrc:/back-arrow-icon.svg"
            anchors.left: parent.left
            height: 20
            anchors.verticalCenter: parent.verticalCenter
            onClicked: sideBarState.state = "newConversationState"
        }

        Common.ButtonForm {
            id: frontbutton
            source: "qrc:/forward-arrow-icon.svg"
            anchors.right: parent.right
            height: 20
            anchors.verticalCenter: parent.verticalCenter
            onClicked: sideBarState.state = "finalizeGroupState"
        }

        Flow {

            topPadding: CmnCfg.smallMargin
            leftPadding: CmnCfg.smallMargin
            anchors.left: backbutton.right
            anchors.right: frontbutton.left

            id: groupFlow
            spacing: CmnCfg.smallMargin / 2

            Repeater {
                id: contactBubbleRepeater
                Keys.enabled: true
                model: groupMemberSelect

                ContactBubble {
                    text: contactsModel.nameById(memberId)
                    userId: memberId
                    defaultColor: CmnCfg.avatarColors[contactsModel.colorById(
                                                          memberId)]

                    Layout.alignment: Qt.AlignLeft
                    xButton.onClicked: groupMemberSelect.removeMemberByIndex(
                                           index)
                }
            }

            TextArea {
                id: searchText
                focus: true
                leftPadding: CmnCfg.smallMargin
                // replace this with a working length property
                placeholderText: "Add people"
                verticalAlignment: TextEdit.AlignVCenter
                background: Rectangle {
                    color: CmnCfg.palette.mainColor
                }

                Keys.onPressed: {
                    // NOTE: What is the first comparison doing?
                    // this makes sure that returns and tabs are not evaluated
                    if (event.key === Qt.Key_Tab) {
                        event.accepted = true
                    }
                    if (event.key === Qt.Key_Backspace && text === "") {
                        groupMemberSelect.removeLast()
                    }

                    if (event.key === Qt.Key_Return) {
                        sideBarState.state = "finalizeGroupState"
                    }
                }

                onTextChanged: {
                    if (contactsSearch) {
                        Qt.callLater(function (text) {
                            contactsModel.filter = text
                        }, searchText.text)
                    } else {
                        Qt.callLater(function (text) {
                            conversationsModel.filter = text
                        }, searchText.text)
                    }
                }
            }
        }
    }
}
