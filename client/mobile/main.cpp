//#include "Bindings.h"
#include <QApplication>
#include <QQmlApplicationEngine>
#include <QtQml/qqml.h>

int main(int argc, char* argv[])
{
  QCoreApplication::setAttribute(Qt::AA_EnableHighDpiScaling);
  QApplication app(argc, argv);

  app.setOrganizationName("Kalix Systems");
  app.setOrganizationDomain("kalix.io");
  app.setApplicationName("Herald");

  QQmlApplicationEngine engine;

  engine.load(QUrl(QStringLiteral("qrc:/qml/main.qml")));
  if (engine.rootObjects().isEmpty()) return -1;

  return app.exec();
}
