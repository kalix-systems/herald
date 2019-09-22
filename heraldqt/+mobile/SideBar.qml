import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.13
import LibHerald 1.0
import "SideBar" as SBUtils
import "common" as Common

// Reveiw Key
// OS Dependent: OSD
// Global State: GS
// Just Hacky: JH
// Type Script: TS
// Needs polish badly: NPB
// Factor Component: FC

Page {
    anchors.fill: parent

    header: SBUtils.UtilityBar {
    }

    Common.ButtonForm {
        id: addContactButton
        anchors {
           right: parent.right
           bottom: parent.bottom
           margins: QmlCfg.margin * 2
        }
        onClicked: print("Todo: push the new conversation view");
        background: Rectangle {
            id: bg
            color: !addContactButton.pressed ? Qt.darker(QmlCfg.palette.tertiaryColor, 1.3) : Qt.darker(QmlCfg.palette.tertiaryColor, 2.5)
            radius: 100
            Image {
                source: "qrc:///icons/plus.png"
                anchors.fill: parent
                scale: 0.9
                mipmap: true
            }

        }
    }



ColumnLayout {
    id: contactPane

    property alias contactsListView: contactsListView
    property alias conversationsListView: conversationsListView
    Layout.fillHeight: true
    Layout.fillWidth: true

    ///--- Contacts View Actual
   ColumnLayout {
        Layout.alignment: Qt.AlignTop
        Layout.fillHeight: true
        Layout.fillWidth:  true

        SBUtils.ContactView {
            id: contactsListView
            Layout.fillHeight: true
            Layout.fillWidth:  true
            model: contactsModel
        }

        SBUtils.ConversationView {
            id: conversationsListView
            Layout.fillHeight: true
            Layout.fillWidth:  true
            model: conversationsModel
        }
    }
  }
}
