import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Dialogs 1.3
import LibHerald 1.0
import "popups" as Popups
import "../common" as Common
import "../common/utils.mjs" as Utils

// Reveiw Key
// OS Dependent: OSD
// Global State: GS
// Just Hacky: JH
// Type Script: TS
// Needs polish badly: NPB
// Factor Component: FC
ToolBar {
    id: toolBar

    anchors {
        left: parent.left
        right: parent.right
        top: parent.top
    }

    height: QmlCfg.toolbarHeight

    background: Rectangle {
        color: QmlCfg.palette.mainColor
        border.color: QmlCfg.palette.secondaryColor
    }

    Common.Avatar {
        id: configAvatar
        displayName: Utils.unwrapOr(config.name, config.configId)
        colorHash: config.color
        // Note: use specific fallback value or implicit one from typescript! TS
        pfpUrl: Utils.unwrapOr(config.profilePicture, "")
        anchors.horizontalCenter: parent.horizontalCenter
        size: parent.height - QmlCfg.margin
    }

    /// unpolished temporary Popup
    /// NPB? Remeber that thing we said abut native right click dialogs?
    Popups.ConfigPopup {
        id: configPopup
    }

    Common.ButtonForm {
        anchors {
            verticalCenter: parent.verticalCenter
            rightMargin: QmlCfg.margin
            right: parent.right
        }
        source: "qrc:///icons/gear.png"
        onClicked: {
            /// NPB this should bring up a new window (impossible from QML) ,
            /// I want an actual config pop up so it feels like i'm using a native app.
            /// Paul should do this from QT widgets Link [https://www.reddit.com/r/Qt5/comments/aoghwr/multiwindow_in_qml_proof_of_concept]
            configPopup.show()
        }
    }
}
