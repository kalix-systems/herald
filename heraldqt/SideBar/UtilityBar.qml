import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Layouts 1.12
import "popups" as Popups
import "../common" as Common
// Reveiw Key
// OS Dependent: OSD
// Global State: GS
// Just Hacky: JH
// Type Script: TS
// Needs polish badly: NPB
// Factor Component: FC
// FS: Fix scoping

// General Note : Baeo

ToolBar {
    id: utilityBar
    anchors.left: parent.left
    anchors.right: parent.right
    height: QmlCfg.toolbarHeight

    background: Rectangle {
        anchors.fill: parent
        color: Qt.darker(QmlCfg.palette.secondaryColor, 1.2)
    }
    // FS: this should be in a lower more specific scope, or maybe a state.*
    // It is coupled with what Icon we use for searching!
    property bool searchRegex: false

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
            /// NPB: what is this, please just find a way to reject the key event
            Keys.onReturnPressed: text = text
            placeholderText: qsTr("Search...")
            Layout.fillWidth: true
            font.pointSize: 12
            onTextChanged: {
                // NOTE: we should probably wrap calls to libherald in call later.
                // this prevents double calls, and is basically a debounce
                Qt.callLater(contactsModel.filter, searchText.text, searchRegex)
            }
        }
    }

    Common.ButtonForm {
        id: searchButton
        anchors {
            right: addContactButton.left
            verticalCenter: parent.verticalCenter
            rightMargin: QmlCfg.margin
        }
        source: "qrc:///icons/search.png"
        onClicked: if (searchRegex) {
                    source = "qrc:///icons/search.png"
                    searchRegex = false
                } else {
                    source = "qrc:///icons/searchRegexTemp.png"
                    searchRegex = true
                }

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
