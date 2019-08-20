import QtQuick 2.13
import QtQuick.Window 2.13
import QtQuick.Layouts 1.3
import QtQuick.Controls 2.13

ApplicationWindow {
    visible: true
    width: 900
    height: 640
    title: qsTr("Herald")
    id : root

    SplitView {

        id: rootSplitView
        anchors.fill: parent
        Layout.fillWidth: true
        Layout.fillHeight: true
        orientation: Qt.Horizontal
        /// Contacts view for the desktop client, in DesktopContacts.qml
        DesktopContacts {  id : contacts  }

        /// placeholder element
     ChatView {  }

    }


}
