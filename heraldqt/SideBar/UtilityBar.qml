import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Layouts 1.12
import "popups" as Popups

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
            leftMargin: 10
            rightMargin: 10
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
            font.pointSize: 10
            onTextChanged: {
                // NOTE: we should probably wrap calls to libherald in call later.
                // this prevents double calls, and is basically a debounce
                Qt.callLater(contactsModel.filter, searchText.text, searchRegex)
            }
        }
    }

    Button {
        id: searchButton
        anchors {
            right: addContactButton.left
            verticalCenter: parent.verticalCenter
            rightMargin: QmlCfg.margin
        }
        height: utilityBar.height - 15
        width: height
        background: Image {
            source: "qrc:///icons/search.png"
            height: width
            scale: 0.9
            mipmap: true
        }
        MouseArea {
            anchors.fill: parent
            acceptedButtons: Qt.RightButton
            //NOTE: This should be coupled to the afforementioned state*
            onClicked: {
                if (searchRegex) {
                    searchButton.background.source = "qrc:///icons/search.png"
                    searchRegex = false
                } else {
                    searchButton.background.source = "qrc:///icons/searchRegexTemp.png"
                    searchRegex = true
                }
            }
        }
    }

    ///--- Add contact button
    Button {
        id: addContactButton
        height: QmlCfg.toolbarHeight - QmlCfg.margin
        width: height

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
                scale: 0.7
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
