import QtQuick 2.14
import QtQuick.Controls 2.14
import QtQuick.Layouts 1.14
import LibHerald 1.0
import "../../"

Column {
    id: wrapper
    width: parent.width

    spacing: CmnCfg.smallMargin
    Row {
        leftPadding: CmnCfg.defaultMargin
        height: language.height
        StandardLabel {
            id: language
            text: qsTr("Language")
            color: CmnCfg.palette.black
                font.family: CmnCfg.chatFont.name
                font.pixelSize: CmnCfg.chatTextSize
        }

        Item {
            height: 10
            width: wrapper.width * 0.66 - language.width
        }

        StandardCombo {
            model: ["English"]
        }
    }

    Rectangle {
        color: CmnCfg.palette.medGrey
        height: 1
        width: parent.width
    }

    Column {
        Row {
            leftPadding: CmnCfg.defaultMargin
            StandardLabel {
                id: app
                text: qsTr("App Info")
                color: "black"
                font.family: CmnCfg.chatFont.name
                font.pixelSize: CmnCfg.chatTextSize
            }

            Item {
                width: wrapper.width * 0.66 - app.width
                height: 10
            }

            GridLayout {
                StandardLabel {
                    text: qsTr("Version ") + "0.0.1-alpha"
                    color: "black"
                    Layout.maximumWidth: wrapper.width * 0.33 - CmnCfg.microMargin
                    wrapMode: Label.WrapAtWordBoundaryOrAnywhere
                font.family: CmnCfg.chatFont.name
                font.pixelSize: CmnCfg.chatTextSize
                }
            }
        }
    }

    Rectangle {
        color: CmnCfg.palette.medGrey
        height: 1
        width: parent.width
    }
}
