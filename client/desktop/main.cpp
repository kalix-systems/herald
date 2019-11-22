#include "Bindings.h"
#include <QApplication>
#include <QQmlApplicationEngine>
#include <QtQml/qqml.h>

int main(int argc, char* argv[])
{
  QCoreApplication::setAttribute(Qt::AA_EnableHighDpiScaling);
  QApplication app(argc, argv);

  qmlRegisterType<Users>("LibHerald", 1, 0, "Users");
  qmlRegisterType<Members>("LibHerald", 1, 0, "Members");
  qmlRegisterType<Messages>("LibHerald", 1, 0, "Messages");
  qmlRegisterType<Conversations>("LibHerald", 1, 0, "Conversations");
  qmlRegisterType<Config>("LibHerald", 1, 0, "Config");
  qmlRegisterType<Herald>("LibHerald", 1, 0, "Herald");
  qmlRegisterType<Utils>("LibHerald", 1, 0, "Utils");
  qmlRegisterType<Errors>("LibHerald", 1, 0, "Errors");
  qmlRegisterType<ConversationBuilder>("LibHerald", 1, 0, "ConversationBuilder");
  qmlRegisterType<MessageBuilder>("LibHerald", 1, 0, "MessageBuilder");
  qmlRegisterType<Attachments>("LibHerald", 1, 0, "Attachments");
  qmlRegisterType<UsersSearch>("LibHerald", 1, 0, "UsersSearch");
  qmlRegisterType<MessageSearch>("LibHerald", 1, 0, "MessageSearch");

  qmlRegisterSingletonType(QUrl("qrc:///common/CommonConfig.qml"), "LibHerald",
                           1, 0, "CmnCfg");

  app.setOrganizationName("Kalix Systems");
  app.setOrganizationDomain("kalix.io");
  app.setApplicationName("Herald");

  QQmlApplicationEngine engine;
  engine.load(QUrl(QStringLiteral("qrc:/main.qml")));
  if (engine.rootObjects().isEmpty()) return -1;

  return app.exec();
}
