import QtQuick 2.13
import QtQuick.Window 2.13
import QtQuick.Layouts 1.12
import QtQuick.Controls 2.13
import LibHerald 1.0
import "SideBar/popups" as Popups
import "common/utils.mjs" as Utils
import QtQml 2.13

Item {
    id: appRoot

    // TODO this can be passed as an argument wherever it's needed
    // PAUL 0: this can be passed as an argument to a C++ helper function.
    // currently the issue is with scoping, instead we can just pass
    // this index like an argument with a dynamic property in C++
    property int gsSelectedIndex: -1

    anchors.fill: parent.fill

    Users {
        id: contactsModel

        onTryPollChanged: {
            pollUpdate()
        }
    }

    Conversations {
        id: conversationsModel
        // PAUL 1: see if we can call connect over FFI to clean these functions up.
        // we could easily do that in `new`.
        onTryPollChanged: {
            pollUpdate()
        }
    }

    Popups.ConfigPopup { id: preferencesPopup }

    Popups.ColorPicker {
        id: avatarColorPicker

        //PAUL 0:  button is here to know index of contact clicked
        // move this inside the color picker after refactor
        Button {
            id: colorSubmissionButton
            text: "Submit"
            anchors {
                right: parent.right
                bottom: parent.bottom
            }

            onClicked: {
                contactsModel.setColor(gsSelectedIndex,
                                       avatarColorPicker.colorIndex)
                avatarColorPicker.close()
            }
        }
    }

    Config {
        id: config
    }

    // PAUL: it is time for real art here.
    Component {
        id: splash
        Image {
            anchors.fill: parent
            source: "qrc:/land.png"
            mipmap: true
        }
    }

    SplitView {
        id: rootSplitView
        anchors.fill: parent
        orientation: Qt.Horizontal

        SideBar {
            id: sideBar
        }

        Loader {
            id: chatView
            sourceComponent: splash
        }

        handle: Rectangle {
            implicitWidth: 1.1
            color: QmlCfg.palette.borderColor
        }
    }

    Component.onCompleted: {
        networkHandle.login()
    }
}
