import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import QtQuick 2.12
import LibHerald 1.0
import "./Controls"

Page {
    id: cvMainView
    header: CVHeader {}
    background: Rectangle {
        color: QmlCfg.palette.mainColor
    }

    // the body of this entire element
    // displays conversations
    ListView {
        id: cvListView
        clip: true
        boundsBehavior: ListView.StopAtBounds
        anchors.fill: parent
        model: 20
        delegate: CVListItem {}
        ScrollBar.vertical: ScrollBar {}
    }

    // floating pencil button to trigger
    // new message flow
    CVFloatingButton {

        anchors {
            bottom: parent.bottom
            right: parent.right
            margins: QmlCfg.margin * 2
        }

        iconSource: "qrc:/pencil-icon-black.svg"
    }
}
