import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.12
import LibHerald 1.0
import Qt.labs.platform 1.0
import "../../"

ColumnLayout {
    RowLayout {
        Layout.fillWidth: true

        StandardLabel {
            text: qsTr("Default message expiration time")
            color: CmnCfg.palette.black
            Layout.leftMargin: CmnCfg.defaultMargin
            Layout.fillWidth: true
            wrapMode: Label.WrapAtWordBoundaryOrAnywhere
            font: CmnCfg.defaultFont
        }

        //TODO: THIS SHOULD COME FROM THE CONFIG MODEL
        StandardCombo {
            model: ["Off", "1 minute", "1 hour", "1 day", "et cetera"]
        }
    }

    Rectangle {
        color: CmnCfg.palette.darkGrey
        height: 1
        Layout.fillWidth: true
    }
}
