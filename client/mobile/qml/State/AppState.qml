import QtQuick 2.12
import QtQuick.Controls 2.12
import "../ConversationView" as CVView
import "../ChatView" as ChatView
import "../LoginPage" as LoginPage

Item {
    property StackView stackView
    // property Component lpMain: LoginPage.LoginLandingPage {}
    property Component cvMain: CVView.ConversationViewMain {}
    property var chatMain: ChatView.ChatViewMain {}

    // list of conversations
    property alias conversationsModel: heraldGlobals.conversationsModel
    property alias usersModel: heraldGlobals.usersModel
    property alias configModel: heraldGlobals.configModel

    HeraldGlobals {
        id: heraldGlobals
    }

    states: [
        State {
            when: heraldState.configInit
            name: "contact"
            StateChangeScript {
                script: {
                    stackView.replace(cvMain)
                    chatMain.ownedMessages = null
                }
            }
        },
        State {
            name: "config"
            PropertyChanges {}
        },
        State {
            name: "chat"
            StateChangeScript {
                script: {
                    stackView.replace(chatMain)
                }
            }
        },
        State {
            name: "Search"
        }
    ]
}
