import QtQuick 2.12
import QtQuick.Controls 2.12
import "../ConversationView" as CVView
import "../ChatView" as ChatView
import "../LoginPage" as LoginPage

Item {
    property StackView stackView
    property Component lpMain: LoginPage.LoginLandingPage {}
    property Component cvMain: CVView.ConversationViewMain {}
    property ChatView.ChatViewMain chatMain: ChatView.ChatViewMain {}

    states: [
        State {
            when: !heraldState.configInit
            name: "setup"

        },
        State {
            when: heraldState.configInit
            name: "contact"
            StateChangeScript {
                script: stackView.replace(cvMain)
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
                    chatMain.ownedMessages = null
                    stackView.replace(chatMain)
                }
            }
        },
        State {
            name: "Search"
        }
    ]
}
