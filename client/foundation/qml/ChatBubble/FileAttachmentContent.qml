import QtQuick 2.12
import QtQuick.Controls 2.12
import QtQuick.Layouts 1.12
import LibHerald 1.0
import QtGraphicalEffects 1.12

ColumnLayout {
    property real maxWidth: Math.min(parent.maxWidth, 600)
    property var docParsed
    id: wrapperCol

    spacing: 0

    Component.onCompleted: {
        const docs = documentAttachments.length === 0 ? "" : JSON.parse(
                                                            documentAttachments)
        const docLen = docs.length
        docParsed = docs

        for (var i in docParsed) {
            docModel.append({
                                "path": docParsed[i].path,
                                "name": docParsed[i].name,
                                "size": docParsed[i].size
                            })
        }

        switch (docLen) {
        case 0:
            break
        default:
            docLoader.sourceComponent = docList
        }
    }

    Loader {
        Layout.rightMargin: CmnCfg.smallMargin
        Layout.leftMargin: CmnCfg.smallMargin
        //Layout.topMargin: CmnCfg.smallMargin
        Layout.bottomMargin: CmnCfg.smallMargin
        id: docLoader
    }

    ListModel {
        id: docModel
    }

    Component {
        id: docList
        DocFileItem {
            height: 20 * docParsed.length
            model: docModel
        }
    }
}
