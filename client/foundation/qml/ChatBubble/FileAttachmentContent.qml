import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import QtGraphicalEffects 1.12
import LibHerald 1.0
import "qrc:/imports" as Imports

// wrapper component for the file list component.
// TODO: move into the DocFileItem, this wrapping is inane
Column {
    id: wrapperCol

    property real maxWidth: Math.min(contentRoot.maxWidth, 600)
    property var docParsed: JSON.parse(documentAttachments).items
    property bool expand: false
    property bool elided: docModel.count > 4

    spacing: 0

    Component.onCompleted: {
        JSON.parse(documentAttachments).items.forEach(function (doc) {
            docModel.append(doc)
        })
    }

    ListModel {
        id: docModel
    }

    DocFileItem {

        model: docModel
    }

    Item {
        visible: elided
        width: elided ? wrapRow.width : 0
        height: elided ? wrapRow.height : 0

        MouseArea {
            anchors.fill: wrapRow
            onClicked: wrapperCol.expand = !wrapperCol.expand
        }

        Row {
            id: wrapRow
            Text {
                id: text
                text: expand ? qsTr("Collapse") : qsTr(
                                   "Show ") + (docModel.count - 4) + qsTr(
                                   " more")
                font.family: CmnCfg.chatFont.name
                font.bold: true
                color: CmnCfg.palette.black
            }

            Imports.IconButton {
                icon.source: !wrapperCol.expand ? "qrc:/up-chevron-icon" : "qrc:/down-chevron-icon"
                fill: CmnCfg.palette.black

                anchors.verticalCenter: text.verticalCenter
                onClicked: wrapperCol.expand = !wrapperCol.expand
                icon.width: 16
            }
        }
    }
}
