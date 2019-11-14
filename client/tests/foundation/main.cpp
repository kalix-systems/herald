#include "Bindings.h"
#include <QQmlApplicationEngine>
#include <QtQml/qqml.h>
#include <QtQuickTest/quicktest.h>

class Setup : public QObject {

public:
  Setup()
  {
    qmlRegisterType<Users>("LibHerald", 1, 0, "Users");
    qmlRegisterType<Members>("LibHerald", 1, 0, "Members");
    qmlRegisterType<Messages>("LibHerald", 1, 0, "Messages");
    qmlRegisterType<Conversations>("LibHerald", 1, 0, "Conversations");
    qmlRegisterType<Config>("LibHerald", 1, 0, "Config");
    qmlRegisterType<HeraldState>("LibHerald", 1, 0, "HeraldState");
    qmlRegisterType<HeraldUtils>("LibHerald", 1, 0, "HeraldUtils");
    qmlRegisterType<Errors>("LibHerald", 1, 0, "Errors");
    qmlRegisterType<ConversationBuilder>("LibHerald", 1, 0,
                                         "ConversationBuilder");
    qmlRegisterType<MessageBuilder>("LibHerald", 1, 0, "MessageBuilder");
    qmlRegisterType<Attachments>("LibHerald", 1, 0, "Attachments");
    qmlRegisterType<MessagePreview>("LibHerald", 1, 0, "MessagePreview");
    qmlRegisterSingletonType(QUrl("qrc:///common/CommonConfig.qml"),
                             "LibHerald", 1, 0, "CmnCfg");
  }

public slots:
  void qmlEngineAvailable(QQmlEngine* engine) { (void)engine; }
};

QUICK_TEST_MAIN_WITH_SETUP(foundation, Setup)
