import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import QtQuick 2.12
import LibHerald 1.0
import "../Headers" as Headers

// TODO this a dummy component, content is only to confirm it gets rendered
Page {
    id: searchView
    readonly property Component headerComponent: Headers.GlobalSearchHeader {}
    background: Rectangle {
        color: CmnCfg.palette.white
        height: 40
        width: 100

        Label {
            text: 'search view'
            anchors.top: parent.top
        }
    }
}
