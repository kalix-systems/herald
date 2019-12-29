import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Layouts 1.12
import "../../common" as Common
import QtGraphicalEffects 1.0
import Qt.labs.platform 1.0
import "qrc:/imports/Entity"
import "qrc:/imports/js/utils.mjs" as Utils

Avatar {
    id: headerAvatar
    color: CmnCfg.palette.avatarColors[Herald.config.color]
    initials: Herald.config.name[0].toUpperCase()
    pfpPath: Utils.safeStringOrDefault(Herald.config.profilePicture, "")
    size: 28


    // TODO onclicked this should open identity switcher, once that exists;
    // uncomment following code once identity switcher is implemented
//    MouseArea {
//        id: avatarHoverHandler
//        anchors.fill: parent

//        cursorShape: Qt.PointingHandCursor

//        onPressed: overlay.visible = true
//        onReleased: overlay.visible = false
//    }

    //TODO dead code?
    ColorOverlay {
        id: overlay
        anchors.fill: parent
        source: parent

        visible: false
        color: "black"
        opacity: 0.2
        smooth: true
    }
}
