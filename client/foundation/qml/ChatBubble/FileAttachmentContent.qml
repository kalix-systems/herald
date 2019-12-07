import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0
import QtGraphicalEffects 1.12

ColumnLayout {
    id: wrapperCol

    property real maxWidth: Math.min(bubbleRoot.maxWidth, 600)
    property var docParsed

    spacing: 0

    Component.onCompleted: {
        if (documentAttachments.length === 0) {
            return
        }

        JSON.parse(documentAttachments).forEach(function (doc) {
            docModel.append(doc)
        })

        docLoader.sourceComponent = docList
    }

    Loader {
        Layout.rightMargin: CmnCfg.smallMargin
        Layout.leftMargin: CmnCfg.smallMargin
        Layout.topMargin: CmnCfg.smallMargin
        Layout.bottomMargin: CmnCfg.smallMargin * 2
        id: docLoader
    }

    ListModel {
        id: docModel
    }

    Component {
        id: docList
        DocFileItem {
            height: 20 * docModel.count
            model: docModel
        }
    }
}
