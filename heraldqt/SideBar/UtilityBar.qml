import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Layouts 1.12
import "popups" as Popups

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
            leftMargin: 10
            rightMargin: 10
            verticalCenter: parent.verticalCenter
        }

        TextArea {
            background: Rectangle {
                anchors.fill: parent
                color: QmlCfg.palette.mainColor
                radius: QmlCfg.radius
            }
            Keys.onReturnPressed: text = text
            id: searchText
            placeholderText: qsTr("Search...")
            Layout.fillWidth: true
            font.pointSize: 10
            onTextChanged: {
                contacts.filter(searchText.text, false)
                                for (var i = 0; i < contacts.rowCount(); i++)
                                   console.log(contacts.name(i), contacts.matched(i))
                              //  contacts.clear_filter()



            }
        }
    }
    Button {
        id: searchButton
        anchors.right: addContactButton.left
        anchors.verticalCenter: parent.verticalCenter
        anchors.rightMargin: QmlCfg.margin
        implicitHeight: utilityBar.height - 15
        implicitWidth: height
        background: Image {
            source: "qrc:///icons/search.png"
            height: width
            scale: 0.9
            mipmap: true
        }
        onClicked:  {contacts.filter(searchText.text, false)
                for (var i = 0; i < contacts.rowCount(); i++)
                    console.log(contacts.name(i) + " " + contacts.matched(i))
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

    Popups.NewContactDialogue {
        id: newContactDialogue
    }
}
