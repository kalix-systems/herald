import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import QtQuick.Layouts 1.12
import "../common" as Common
import QtGraphicalEffects 1.0
import Qt.labs.platform 1.0
import "qrc:/imports/Avatar"
import "qrc:/imports/js/utils.mjs" as Utils

// TODOs for this file:
// - rename something like HeaderAvatar or IdentityAvatar
// - move to /Sidebar/header
// - clicking this avatar should open the "identity switcher," a thing we
//   haven't designed yet; for now clicking on it does nothing
//common desktop config avatar with overlay
AvatarMain {
    id: configAvatar
    backgroundColor: CmnCfg.palette.avatarColors[Herald.config.color]
    initials: Herald.config.name[0].toUpperCase()
    pfpPath: Utils.safeStringOrDefault(Herald.config.profilePicture, "")

    size: 28
    avatarHeight: 28

    MouseArea {
        id: avatarHoverHandler
        anchors.fill: parent

        cursorShape: Qt.PointingHandCursor

        onPressed: overlay.visible = true
        onReleased: overlay.visible = false
        onClicked: configPopup.show()
    }

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
