import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Controls.Styles 1.4
import QtGraphicalEffects 1.13
import LibHerald 1.0

ToolBar {
    visible: false
    anchors {
        left: parent.left
        right: parent.right
        top: parent.top
    }

    height: 0 //QmlCfg.toolbarHeight

    background: Rectangle {
        color: QmlCfg.palette.mainColor
        border.color: QmlCfg.palette.secondaryColor
    }
    Switch {
        id: control

        indicator: Rectangle {
            id: toggleStyle
            anchors.centerIn: parent
            implicitWidth: QmlCfg.margin * 10
            implicitHeight: 35
            radius: QmlCfg.radius
            border.color: QmlCfg.palette.secondaryColor
            border.width: 0.5
            Image {
                source: control.checked ? "qrc:/speech_hollow.png" : "qrc:/speech_filled.png"
                mipmap: true
                anchors {
                    top: parent.top
                    bottom: parent.bottom
                    left: parent.left
                    right: parent.horizontalCenter
                    rightMargin: QmlCfg.smallMargin
                    leftMargin: QmlCfg.smallMargin
                }
            }
            Image {
                source: control.checked ? "qrc:/cont_filled.png" : "qrc:/cont_hollow.png"
                mipmap: true
                anchors {
                    top: parent.top
                    bottom: parent.bottom
                    left: parent.horizontalCenter
                    right: parent.right
                    rightMargin: QmlCfg.smallMargin
                    leftMargin: QmlCfg.smallMargin
                }
            }
        }

        anchors.centerIn: parent
    }
}
