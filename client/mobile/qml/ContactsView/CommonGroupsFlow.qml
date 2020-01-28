import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.13
import LibHerald 1.0
import "qrc:/imports"
import "qrc:/imports/Entity"
import "qrc:/imports/js/utils.mjs" as Utils
import QtGraphicalEffects 1.0

Flow {
    spacing: CmnCfg.microMargin

    Repeater {
        model: userRect.sharedConvos
        delegate: Avatar {
            id: groupAv
            property var groupData: ContentMap.get(model.conversationId)
            size: CmnCfg.units.dp(22)
            isGroup: true
            // TODO try making this 8 and see if any phones are too small
            visible: index < 6

            property int groupColor: groupData.conversationColor
                                     !== undefined ? groupData.conversationColor : 0
            pfpPath: Utils.safeStringOrDefault(groupData.picture, "")

            color: CmnCfg.avatarColors[groupColor]
            initials: Utils.initialize(Utils.safeStringOrDefault(
                                           groupData.title))

            TapHandler {
                enabled: !overlay.visible
                onTapped: groupClicked(groupData.conversationId)
            }

            Rectangle {
                anchors.fill: parent
                color: "transparent"

                id: overlay
                visible: (userRect.sharedConvos.rowCount() > 6 && index === 5)

                ColorOverlay {
                    anchors.fill: parent
                    color: "black"
                    opacity: 0.5
                }
                TapHandler {
                    onTapped: {
                        contactPage.userData = userData
                        stackView.push(contactPage)
                    }
                }

                Label {
                    anchors.centerIn: parent
                    text: "+" + (userRect.sharedConvos.rowCount() - 6)
                    color: CmnCfg.palette.white
                    font.family: CmnCfg.chatFont.name
                    font.weight: Font.Medium
                    font.pixelSize: CmnCfg.minorTextSize
                }
            }
        }
    }
}
