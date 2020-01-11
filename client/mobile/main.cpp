#include "Bindings.h"
#include "objectiveutils.h"
#include "androidhelper.h"
#include <QScreen>
#include <QQmlContext>
#include <QApplication>
#include <QQmlApplicationEngine>
#include <QStandardPaths>
#include <QtQml/qqml.h>

int main(int argc, char* argv[])
{


  QCoreApplication::setAttribute(Qt::AA_EnableHighDpiScaling);
  QApplication::setOrganizationName("Kalix Systems");
  QApplication::setOrganizationDomain("kalix.io");
  QApplication::setApplicationName("Herald");

  QApplication app(argc, argv);

  qmlRegisterSingletonType<Herald>(
      "LibHerald", 1, 0, "Herald",
      [](QQmlEngine* engine, QJSEngine* scriptEngine) {
        Q_UNUSED(engine)
        Q_UNUSED(scriptEngine)

        QStandardPaths::StandardLocation local =
            QStandardPaths::AppDataLocation;

        QString path = QStandardPaths::writableLocation(local);

        Herald* state = new Herald();
        state->setAppLocalDataDir(path);

        return state;
      });

  qmlRegisterAnonymousType<Users>("LibHerald", 1);
  qmlRegisterAnonymousType<Config>("LibHerald", 1);
  qmlRegisterAnonymousType<Utils>("LibHerald", 1);
  qmlRegisterAnonymousType<Errors>("LibHerald", 1);
  qmlRegisterAnonymousType<ConversationBuilder>("LibHerald", 1);
  qmlRegisterAnonymousType<UsersSearch>("LibHerald", 1);
  qmlRegisterAnonymousType<MessageSearch>("LibHerald", 1);
  qmlRegisterAnonymousType<Conversations>("LibHerald", 1);

  qmlRegisterType<ConversationContent>("LibHerald", 1, 0,
                                       "ConversationContent");
  qmlRegisterType<Messages>("LibHerald", 1, 0, "Messages");
  qmlRegisterAnonymousType<Members>("LibHerald", 1);
  qmlRegisterAnonymousType<MessageBuilder>("LibHerald", 1);
  qmlRegisterAnonymousType<MediaAttachments>("LibHerald", 1);
  qmlRegisterAnonymousType<DocumentAttachments>("LibHerald", 1);

  qmlRegisterSingletonType(QUrl("qrc:/qml/Common/CommonConfig.qml"),
                           "LibHerald", 1, 0, "CmnCfg");


#ifdef Q_OS_IOS
  qmlRegisterType<ObjectiveUtils>("LibHerald", 1, 0, "MobileHelper");
#elif defined Q_OS_ANDROID
  qmlRegisterType<AndroidHelper>("LibHerald", 1, 0, "MobileHelper");
#else
  qmlRegisterType<QObject>("LibHerald", 1, 0, "MobileHelper");
#endif
  QQmlApplicationEngine engine;


  engine.load(QUrl(QStringLiteral("qrc:/qml/main.qml")));
  if (engine.rootObjects().isEmpty()) return -1;
  return QApplication::exec();
}
