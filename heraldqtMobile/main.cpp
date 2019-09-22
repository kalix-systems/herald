#include "Bindings.h"
#include <QApplication>
#include <QQmlApplicationEngine>
#include <QtQml/qqml.h>

int main(int argc, char* argv[])
{
  QCoreApplication::setAttribute(Qt::AA_EnableHighDpiScaling);
  QApplication app(argc, argv);

  qmlRegisterType<Users>("LibHerald", 1, 0, "Users");
  qmlRegisterType<Messages>("LibHerald", 1, 0, "Messages");
  qmlRegisterType<Conversations>("LibHerald", 1, 0, "Conversations");
  qmlRegisterType<Config>("LibHerald", 1, 0, "Config");
  qmlRegisterType<NetworkHandle>("LibHerald", 1, 0, "NetworkHandle");
  qmlRegisterType<HeraldState>("LibHerald", 1, 0, "HeraldState");
  qmlRegisterType<HeraldUtils>("LibHerald", 1, 0, "HeraldUtils");
  qmlRegisterSingletonType(QUrl("qrc:///common/CommonConfig.qml"), "LibHerald",
                           1, 0, "QmlCfg");

  app.setOrganizationName("Kalix Systems");
  app.setOrganizationDomain("kalix.io");
  app.setApplicationName("Herald");

  QQmlApplicationEngine engine;
  engine.load(QUrl(QStringLiteral("qrc:/main.qml")));
  if (engine.rootObjects().isEmpty()) return -1;

  return app.exec();
}
