import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import QtQuick 2.12
import LibHerald 1.0

// TODO this a dummy component, content is only to confirm it gets rendered
Page {
    id: searchView
    //header: GlobalSearchHeader {}
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
