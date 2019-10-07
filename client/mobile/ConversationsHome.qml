import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0

Page {
    //header bar
    header: UtilityBar {
        buttonSpacing: 15
        marginWidth: QmlCfg.margin
        iconDimLarge: 30
        iconDimSmall: 20
        toolBarHeight: 50
        secondary: QmlCfg.palette.secondaryColor

        drawerButton {
            onClicked: homeDrawer.open()
        }

        searchButton {
            onClicked: {

            }
        }
        toggleButton {
            onClicked: {

            }
        }
        menuButton {
            onClicked: {

            }
        }
    }

    // floating compose message button
    Button {
        id: composeButton
        height: 60
        width: height
        anchors {
            right: parent.right
            bottom: parent.bottom
            margins: 20
        }
        background: Rectangle {
            radius: composeButton.height
            color: QmlCfg.palette.secondaryColor
            anchors.fill: parent
            Image {
                source: "plus-icon.svg"
                sourceSize: Qt.size(48, 48)
                anchors.fill: parent
            }
        }
    }

    Drawer {
        id: homeDrawer
        width: 0.66 * parent.width
        height: parent.height
        dragMargin: QmlCfg.margin * 2

        TeamColumn {
            id: teamColumn
            height: parent.height
        }

        Rectangle {
            anchors.left: teamColumn.right
            color: "black"
            height: parent.height
            width: 1
        }
    }

    // contact view
}
