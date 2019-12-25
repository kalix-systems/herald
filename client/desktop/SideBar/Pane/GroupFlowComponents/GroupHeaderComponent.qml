import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Layouts 1.12
import "../../../common" as Common
import "qrc:/imports" as Imports
import "qrc:/imports/NewGroupFlow" as NewGroupFlow
import QtQuick.Dialogs 1.3
import QtMultimedia 5.13
import QtGraphicalEffects 1.0

//TODO: RENAME this file to ImageSelector or something.
Rectangle {
    id: topRect
    anchors.top: parent.top
    height: 70
    width: parent.width
    color: CmnCfg.palette.offBlack
    property alias profPic: imageSelector.imageSource

    Row {
        height: 42
        anchors.top: parent.top
        anchors.topMargin: CmnCfg.megaMargin
        anchors.horizontalCenter: parent.horizontalCenter
        spacing: CmnCfg.megaMargin

        NewGroupFlow.GroupImageSelector {
            id: imageSelector
            imageSource: topRect.profPic
            color: CmnCfg.palette.white
        }
    }
}
