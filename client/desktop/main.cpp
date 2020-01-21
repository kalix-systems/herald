#include "Bindings.h"
#include "conversationmap.h"
#include <QApplication>
#include <QQmlApplicationEngine>
#include <QStandardPaths>
#include <QtQml/qqml.h>
#include <QWindow>
#include <QDebug>

int main(int argc, char* argv[])
{
  QCoreApplication::setAttribute(Qt::AA_EnableHighDpiScaling);

  QApplication::setOrganizationName("Kalix Systems");
  QApplication::setOrganizationDomain("kalix.io");
  QApplication::setApplicationName("Herald");
  QApplication app(argc, argv);
  QIcon icon = QIcon(":/herald.png");
  QApplication::setWindowIcon(icon);

  qmlRegisterSingletonType<Herald>(
      "LibHerald", 1, 0, "Herald",
      [](QQmlEngine* engine, QJSEngine* scriptEngine) {
        Q_UNUSED(engine)
        Q_UNUSED(scriptEngine)

        auto local = QStandardPaths::AppDataLocation;
        QString path = QStandardPaths::writableLocation(local);


        Herald* state = new Herald();
        state->setAppLocalDataDir(path);


        return state;
      });

  qmlRegisterSingletonType<ConversationMap>(
      "LibHerald", 1, 0, "ContentMap",
      [](QQmlEngine* engine, QJSEngine* scriptEngine) {
        Q_UNUSED(scriptEngine)

        ConversationMap* contentMap = new ConversationMap();
        engine->setObjectOwnership(contentMap, QQmlEngine::CppOwnership);

        return contentMap;
      });

  qmlRegisterAnonymousType<Users>("LibHerald", 1);
  qmlRegisterAnonymousType<Config>("LibHerald", 1);
  qmlRegisterAnonymousType<Utils>("LibHerald", 1);
  qmlRegisterAnonymousType<Errors>("LibHerald", 1);
  qmlRegisterAnonymousType<ConversationBuilder>("LibHerald", 1);
  qmlRegisterAnonymousType<UsersSearch>("LibHerald", 1);
  qmlRegisterAnonymousType<MessageSearch>("LibHerald", 1);
  qmlRegisterAnonymousType<Conversations>("LibHerald", 1);
  qmlRegisterAnonymousType<Messages>("LibHerald", 1);
  qmlRegisterAnonymousType<Members>("LibHerald", 1);
  qmlRegisterAnonymousType<MessageBuilder>("LibHerald", 1);
  qmlRegisterAnonymousType<MediaAttachments>("LibHerald", 1);
  qmlRegisterAnonymousType<DocumentAttachments>("LibHerald", 1);

  qmlRegisterAnonymousType<ConversationContent>("LibHerald", 1);

  qmlRegisterType<SharedConversations>("LibHerald", 1, 0,
                                       "SharedConversations");
  qmlRegisterType<EmojiPicker>("LibHerald", 1, 0, "EmojiPicker");

  qmlRegisterSingletonType(QUrl("qrc:///common/CommonConfig.qml"), "LibHerald",
                           1, 0, "CmnCfg");

  QQmlApplicationEngine engine;


  engine.load(QUrl(QStringLiteral("qrc:/main.qml")));
  if (engine.rootObjects().isEmpty()) return -1;

  return QApplication::exec();
}
