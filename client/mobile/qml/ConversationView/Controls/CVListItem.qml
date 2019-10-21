import QtQuick 2.13
import QtQuick.Controls 2.13
import LibHerald 1.0
import "qrc:/imports/Avatar"

Rectangle {
    id: contactItem

    // the group name or displayName of the conversation
    property string contactName
    // the previous message of the conversation, or the empty string
    property string body
    // the previous latest human readable timestamp, or the empty string
    property string timestamp
    // the value of the latest read receipt according to the ReceiptStatus enum
    property int lastReceipt: 0
    // the index corresponding to the visual color of this GroupBox
    property int colorCode: 0

    height: QmlCfg.units.dp(60)
    color: QmlCfg.palette.mainColor
    border.color: QmlCfg.palette.secondaryColor

    clip: true
    // fill parent width
    anchors {
        left: parent.left
        right: parent.right
    }

    AvatarMain {
        iconColor: QmlCfg.avatarColors[colorCode]
        anchors.verticalCenter: parent.verticalCenter
        initials: initialize("George Michael")
        labelComponent: ConversationLabel {
            contactName: "George Michael"
            lastBody: "Body"
            lastTimestamp: "Wed 2:30"
            lastReceipt: 2
        }
    }

    // background item which gets manipulated
    // during the on tap animation
    Rectangle {
        id: splash
        width: 0
        height: width
        color: "#aaaaaa"
        opacity: 0
        radius: width
        transform: Translate {
            x: -splash.width / 2
            y: -splash.height / 2
        }
    }

    ParallelAnimation {
        id: splashAnim
        NumberAnimation {
            target: splash
            property: "width"
            duration: QmlCfg.units.longDuration
            easing.type: Easing.InOutQuad
            from: parent.width / 2
            to: parent.width * 2
        }
        NumberAnimation {
            target: splash
            property: "opacity"
            duration: QmlCfg.units.longDuration
            easing.type: Easing.InOutQuad
            from: 0.2
            to: 0
        }
    }

    TapHandler {
        onTapped: {
            splash.x = eventPoint.position.x
            splash.y = eventPoint.position.y
            splashAnim.running = true
        }
    }

    function initialize(name) {
        const tokens = name.split(' ').slice(0, 3)
        var str = ""
        tokens.forEach(function anon(string) {
            str += string[0].toUpperCase()
        })
        return str
    }
}
