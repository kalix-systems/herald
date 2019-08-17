import QtQuick 2.0
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.4

    Pane {
        property real windowFraction: 0.25                       // By default set the width to 1/4 the total window size.
        property bool isContactOnlyView: false                   // set true if the only view is the contact list
        property real maxWindowFraction:  0.66

        SplitView.maximumWidth: root.width * maxWindowFraction   // maximum width, where root is ApplicationWindow
        SplitView.minimumWidth: 250
        SplitView.preferredWidth: root.width * windowFraction

        onWidthChanged: { windowFraction = width / root.width; }

        id: contactPane
        padding: 0                                 // All Interior Elements span the entire pane
        height: parent.height
        background: Rectangle {
            border.color: "#AFAFAF"
        }

///--- DesktopContacts Contents area

///--- Username and Settings gear button
            ToolBar {
                id: toolBar
                anchors.left: parent.left
                anchors.top: parent.top
                width: contactPane.width
                height: 40
                background: Rectangle {
                    color: "#EFEFEF"
                    border.color: "#AFAFAF"
                }
            }

///--- SearchBar for contacts, add contact button
                ToolBar {
                    id: searchBar
                    anchors.left: parent.left
                    y: toolBar.y + toolBar.height
                    width: contactPane.width
                    height: 40
                    font.pointSize: 25
                    background: Rectangle {
                        anchors.fill : parent
                        color: "#FFFFFF"
                    }


                    ///--- Add contact button
                    Button {
                        id: addContactButton
                        font.pointSize: parent.height - 10
                        height : parent.height - 15
                        anchors.rightMargin: 10
                        anchors.verticalCenterOffset: 0
                        anchors.right: parent.right
                        anchors.verticalCenter: parent.verticalCenter
                        width : height

                        background:  Rectangle {
                            id : bg
                            color: "#3c7c9b"
                            radius: 100
                            Image {
                                source: "icons/plus.png"
                                anchors.fill: parent
                                scale: 0.7
                            }
                        }

                        MouseArea {
                            anchors.fill: parent;
                            hoverEnabled: true
                            onEntered: {  bg.color = Qt.darker(bg.color, 1.5);  }
                            onExited: { bg.color = Qt.lighter(bg.color, 1.5);  }
                            onPressed: {  bg.color = Qt.darker(bg.color, 2.5);  }
                            onReleased: {  bg.color = Qt.lighter(bg.color, 2.5);  }
                        }
                    }


                }
///--- Border between SearchBar and the Pane Contents (contacts)
            Rectangle {
                id: searchBarBorder
                anchors.top: searchBar.bottom
                color: "#AFAFAF"
                width: parent.width
                height: 1
            }

///--- Contacts View Actual
            Pane {
                anchors.right: parent.right
                anchors.left: parent.left
                anchors.top : searchBarBorder.bottom
                anchors.bottom : parent.bottom
                ContactView {
                     anchors.fill : parent
                }
            }


    }



/*##^## Designer {
    D{i:10;anchors_height:1}
}
 ##^##*/
