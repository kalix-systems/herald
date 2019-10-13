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
        color: QmlCfg.palette.mainColor

        Common.ButtonForm {
            id: backbutton
            source: "qrc:/back-arrow-icon.svg"
            anchors.left: parent.left
            height: 20
            anchors.verticalCenter: parent.verticalCenter
            onClicked: convoPane.state = "newConversationState"
        }

        Common.ButtonForm {
            id: frontbutton
            source: "qrc:/forward-arrow-icon.svg"
            anchors.right: parent.right
            height: 20
            anchors.verticalCenter: parent.verticalCenter
            onClicked: convoPane.state = "finalizeGroupState"
        }

        Flow {

            topPadding: QmlCfg.smallMargin
            leftPadding: QmlCfg.smallMargin
            anchors.left: backbutton.right
            anchors.right: frontbutton.left

            id: groupFlow
            spacing: QmlCfg.smallMargin / 2

            Repeater {
                id: contactBubbleRepeater
                Keys.enabled: true
                model: groupMemberSelect

                SBUtils.ContactBubble {
                    text: contactsModel.nameById(memberId)
                    userId: memberId
                    defaultColor: QmlCfg.avatarColors[contactsModel.colorById(
                                                          memberId)]

                    Layout.alignment: Qt.AlignLeft
                    xButton.onClicked: groupMemberSelect.removeMemberByIndex(
                                           index)
                }
            }

            TextArea {
                id: searchText
                focus: true
                leftPadding: QmlCfg.smallMargin

                // BNOTE: I don't think count works?
                placeholderText: if (groupMemberSelect.count === 0)
                                     "Add people"
                                 else
                                     ""
                verticalAlignment: TextEdit.AlignVCenter
                background: Rectangle {
                    color: QmlCfg.palette.mainColor
                }

                //PAUL: move this logic to C++
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
                        convoPane.state = "finalizeGroupState"
                    }
                }

                //PAUL: move this logic to C++
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
