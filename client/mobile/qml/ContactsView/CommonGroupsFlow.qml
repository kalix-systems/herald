import QtQuick 2.13
import QtQuick.Controls 2.13
import QtQuick.Layouts 1.13
import LibHerald 1.0
import "qrc:/imports"
import "qrc:/imports/Entity"
import "qrc:/imports/js/utils.mjs" as Utils
import QtGraphicalEffects 1.0

Flow {
    anchors.horizontalCenter: parent.horizontalCenter
    anchors.verticalCenter: parent.verticalCenter
    spacing: CmnCfg.microMargin
    width: CmnCfg.units.dp(66)

    Repeater {
        model: userRect.sharedConvos
        delegate: Avatar {
            id: groupAv
            property var groupData: model
            size: CmnCfg.units.dp(20)
            isGroup: true
            visible: index < 6

            property int groupColor: groupData.conversationColor
                                     !== undefined ? groupData.conversationColor : 0
            pfpPath: Utils.safeStringOrDefault(groupData.conversationPicture,
                                               "")

            color: CmnCfg.avatarColors[groupColor]
            initials: Utils.initialize(Utils.safeStringOrDefault(
                                           groupData.conversationTitle))

            MouseArea {
                enabled: !overlay.visible
                anchors.fill: parent
                cursorShape: Qt.PointingHandCursor
                hoverEnabled: true
            }

            Rectangle {
                anchors.fill: parent
                color: "transparent"

                id: overlay
                visible: (userRect.sharedConvos.rowCount() > 5 && index === 5)
                ColorOverlay {
                    anchors.fill: parent
                    color: "black"
                    opacity: 0.5
                }
                TapHandler {
                    onTapped: {
                        drawer.userData = userData
                        drawer.open()
                    }
                }

                Label {
                    anchors.centerIn: parent
                    text: "+" + (userRect.sharedConvos.rowCount() - 6)
                    color: "white"
                    font.family: CmnCfg.chatFont.name
                    font.weight: Font.DemiBold
                    font.pixelSize: CmnCfg.defaultFontSize
                }
            }
        }
    }
}
