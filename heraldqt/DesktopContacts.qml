import QtQuick 2.0
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.13
import LibHerald 1.0


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
                            onClicked: { newContactDialogue.open(); }
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
            Contacts { id : contacts }
            Pane {
                padding: 0
                anchors.right: parent.right
                anchors.left: parent.left
                anchors.top : searchBarBorder.bottom
                anchors.bottom : parent.bottom
                ContactView {
                     anchors.fill : parent
                     model : contacts
                }
            }

///--- popup dialog containing contact insertion UI
            function insertContact() {
                if ( entryField.text.trim().length == 0 ) {return}
                contacts.add(entryField.text.trim());
                entryField.clear();
                newContactDialogue.close();
            }

            Popup {
                id: newContactDialogue
                modal: true
                focus: true
                closePolicy: Popup.CloseOnEscape | Popup.CloseOnPressOutside
                width: 500
                height: 300


                Text {
                    text: "this is a placeholder, forgive me"
                    anchors.bottom: parent.bottom
                }

                TextArea {
                    focus : true
                    id : entryField
                    placeholderText: qsTr("Enter contact name")
                    Keys.onReturnPressed: insertContact("");
                }

                Button {
                    text: "submit"
                    id: submissionButton
                    anchors.bottom: parent.bottom
                    anchors.right: parent.right
                    onClicked: insertContact("");
                }

            }



    }



/*##^## Designer {
    D{i:10;anchors_height:1}
}
 ##^##*/
