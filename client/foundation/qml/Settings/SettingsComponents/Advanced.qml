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
            font: CmnCfg.defaultFont
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
            //  height: app.height
            leftPadding: CmnCfg.defaultMargin
            StandardLabel {
                id: app
                text: qsTr("App Info")
                color: "black"
                font: CmnCfg.defaultFont
            }

            Item {
                width: wrapper.width * 0.66 - app.width
                height: 10
            }

            GridLayout {
                StandardLabel {
                    text: qsTr("Version ") + "0.0.1-alpha"
                    color: "black"
                    font: CmnCfg.defaultFont
                    Layout.maximumWidth: wrapper.width * 0.33 - CmnCfg.microMargin
                    wrapMode: Label.WrapAtWordBoundaryOrAnywhere
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
