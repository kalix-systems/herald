import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import LibHerald 1.0
import Qt.labs.platform 1.0
import "../../"

Column {
    width: parent.width
    spacing: CmnCfg.smallMargin
    id: wrapper
    Row {
        width: parent.width
        height: label.height
        leftPadding: CmnCfg.defaultMargin
        GridLayout {
            Label {
                id: label
                text: qsTr("Default message expiration for new conversations")
                Layout.maximumWidth: wrapper.width * 0.66 - CmnCfg.smallMargin
                color: CmnCfg.palette.black
                wrapMode: Label.WrapAtWordBoundaryOrAnywhere
                font: CmnCfg.defaultFont
            }
        }

        Item {
            height: 10
            width: wrapper.width * 0.66 - label.width
        }
        //TODO: THIS SHOULD COME FROM THE CONFIG MODEL
        StandardCombo {
            id: combo
            model: ["Off", "1 minute", "1 hour", "1 day", "et cetera"]
        }
    }

    Rectangle {
        color: CmnCfg.palette.medGrey
        height: 1
        width: parent.width
    }
}
