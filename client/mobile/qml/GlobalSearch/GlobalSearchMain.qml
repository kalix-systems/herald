import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import QtQuick 2.12
import LibHerald 1.0

// TODO this a dummy component, content is only to confirm it gets rendered
Page {
    id: searchView
    readonly property Component headerComponent: GlobalSearchHeader {}
    property Loader headerLoader

    background: Rectangle {
        color: CmnCfg.palette.white
        height: 40
        width: 100

        Button {
            text: 'go'
            onClicked: {
                print(headerLoader.item.searchText)
                print(Herald.conversations)
            }
        }
    }
}
