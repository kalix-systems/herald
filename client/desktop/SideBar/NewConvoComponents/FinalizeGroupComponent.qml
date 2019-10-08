import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.13
import LibHerald 1.0
import "../../common" as Common

Component {
    id: finalizeGroupComponent

    Rectangle {
        height: QmlCfg.toolbarHeight
        width: parent.width

        Common.ButtonForm {
            id: finalizegroupbutton
            source: "qrc:/single-check-receipt-icon.svg"
            anchors.right: parent.right
            anchors.verticalCenter: parent.verticalCenter
            scale: 0.8
            onClicked: {
                groupMemberSelect.setTitle(groupTitle.text)
                groupMemberSelect.finalize()
                convoPane.state = ""
            }
        }

        Common.ButtonForm {
            id: backbutton
            source: "qrc:/back-arrow-icon.svg"
            anchors.left: parent.left
            anchors.verticalCenter: parent.verticalCenter
            scale: 0.8
            onClicked: {
                convoPane.state = "newGroupState"
            }
        }

        TextArea {

            id: groupTitle
            height: parent.height
            anchors.left: backbutton.right
            anchors.right: finalizegroupbutton.left

            placeholderText: "Group title"

            verticalAlignment: TextEdit.AlignVCenter
            background: Rectangle {
                color: QmlCfg.palette.mainColor
                anchors.fill: parent
            }

            Keys.onPressed: {
                // NOTE: What is the first comparison doing?
                // this makes sure that returns and tabs are not evaluated
                if (event.key === Qt.Key_Tab) {
                    event.accepted = true
                }

                if (event.key === Qt.Key_Return) {
                    groupMemberSelect.setTitle(groupTitle.text)
                    groupMemberSelect.finalize()
                    convoPane.state = ""
                }
            }
        }
    }
}
