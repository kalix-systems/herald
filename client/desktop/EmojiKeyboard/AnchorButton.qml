import QtQuick 2.13
import QtQuick.Controls 2.13
import QtGraphicalEffects 1.0
import LibHerald 1.0

Button {
    property string imageSource: ""
    // the sections names are kept in the anchor buttons
    // as an impromptu list model.
    // TODO: make a real list model which initializes
    // the anchor buttons
    property string sectionName: ""

    onClicked: listLoader.position(index)
    padding: 0
    background: Item {}
    icon.source: imageSource
    icon.color: CmnCfg.palette.white
    icon.width: 15
    icon.height: 15
}
