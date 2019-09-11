import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Layouts 1.12
import "popups" as Popups
import "../common" as Common
import "../common/utils.mjs" as Utils

// Reveiw Key
// OS Dependent: OSD
// Global State: GS
// Just Hacky: JH
// Type Script: TS
// Needs polish badly: NPB
// Factor Component: FC
// FS: Fix scoping

ToolBar {
    id: utilityBar
    anchors.left: parent.left
    anchors.right: parent.right
    height: QmlCfg.toolbarHeight

    background: Rectangle {
        anchors.fill: parent
        color: Qt.darker(QmlCfg.palette.secondaryColor, 1.2)
    }

    ScrollView {
        id: searchScroll
        anchors {
            left: parent.left
            right: searchButton.left
            leftMargin: QmlCfg.margin
            rightMargin: QmlCfg.margin
            verticalCenter: parent.verticalCenter
        }

        TextArea {
            id: searchText
            background: Rectangle {
                anchors.fill: parent
                color: QmlCfg.palette.mainColor
                radius: QmlCfg.radius
            }
            Keys.onPressed: {
                // NOTE: What is the first comparison doing?
                if (event.key === Qt.Key_Return) {
                    event.accepted = true
                } else if (event.key === Qt.Key_Tab) {
                    event.accepted = true
                }
            }
            selectionColor: QmlCfg.palette.tertiaryColor
            placeholderText: qsTr("Search...")
            Layout.fillWidth: true
            font.pointSize: 12
            onTextChanged: {
                Qt.callLater((text) => { contactsModel.filter = text }, searchText.text)
            }
        }
    }

    Common.ButtonForm {
        id: searchButton
        property bool searchRegex: false
        anchors {
            right: addContactButton.left
            verticalCenter: parent.verticalCenter
            rightMargin: QmlCfg.margin
        }
        source: Utils.safeSwitch(searchRegex,
                                 "qrc:///icons/searchRegexTemp.png",
                                 "qrc:///icons/search.png")
        onClicked: searchRegex = contactsModel.toggleFilterRegex()
    }

    ///--- Add contact button
    Common.ButtonForm {
        id: addContactButton
        anchors {
            rightMargin: QmlCfg.margin
            verticalCenterOffset: 0
            right: parent.right
            verticalCenter: parent.verticalCenter
        }

        background: Rectangle {
            id: bg
            color: Qt.darker(QmlCfg.palette.tertiaryColor, 1.3)
            radius: 100
            Image {
                source: "qrc:///icons/plus.png"
                anchors.fill: parent
                scale: 0.9
                mipmap: true
            }
        }

        // NPB: States
        MouseArea {
            anchors.fill: parent
            hoverEnabled: true
            onEntered: {
                bg.color = Qt.darker(bg.color, 1.5)
            }
            onExited: {
                bg.color = Qt.lighter(bg.color, 1.5)
            }
            onPressed: {
                bg.color = Qt.darker(bg.color, 2.5)
            }
            onReleased: {
                bg.color = Qt.lighter(bg.color, 2.5)
            }
            onClicked: {
                newContactDialogue.open()
            }
        }
    }

    //NOTE: see previous notes about using native dialogs
    Popups.NewContactDialogue {
        id: newContactDialogue
    }
}
