#include "Bindings.h"
#include "androidhelper.h"
#include "contentmap.h"
#include "objectiveutils.h"
#include "usermap.h"
#include "qqqmlclipboard.h"
#include <QApplication>
#include <QQmlApplicationEngine>
#include <QQmlContext>
#include <QScreen>
#include <QStandardPaths>
#include <QtQml/qqml.h>

int main(int argc, char* argv[])
{
  QCoreApplication::setAttribute(Qt::AA_EnableHighDpiScaling);
  QApplication::setOrganizationName("Kalix Systems");
  QApplication::setOrganizationDomain("kalix.io");
  QApplication::setApplicationName("Herald");

  QApplication app(argc, argv);

  // Main app state
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

  auto errMsg = [](QString type, QString access) {
    return (type + " should not be created from QML, access through " + access);
  };

  auto heraldMsg = [errMsg](QString type) { return errMsg(type, "Herald"); };

  qmlRegisterUncreatableType<Users>("LibHerald", 1, 0, "Users",
                                    heraldMsg("Users"));

  // Provides access to user information
  qmlRegisterSingletonType<UserMap>(
      "LibHerald", 1, 0, "UserMap",
      [](QQmlEngine* engine, QJSEngine* scriptEngine) {
        Q_UNUSED(scriptEngine)
        Q_UNUSED(engine)

        UserMap* userMap = new UserMap();

        return userMap;
      });

  qmlRegisterUncreatableType<User>("LibHerald", 1, 0, "User",
                                   errMsg("User", "UserMap"));

  qmlRegisterUncreatableType<Config>("LibHerald", 1, 0, "Config",
                                     heraldMsg("Config"));

  qmlRegisterUncreatableType<Utils>("LibHerald", 1, 0, "Utils",
                                    heraldMsg("Utils"));

  qmlRegisterUncreatableType<Errors>("LibHerald", 1, 0, "Errors",
                                     heraldMsg("Errors"));

  qmlRegisterUncreatableType<ConversationBuilder>(
      "LibHerald", 1, 0, "ConversationBuilder",
      heraldMsg("ConversationBuilder"));

  qmlRegisterUncreatableType<UsersSearch>("LibHerald", 1, 0, "UsersSearch",
                                          heraldMsg("UsersSearch"));

  qmlRegisterUncreatableType<MessageSearch>("LibHerald", 1, 0, "MessageSearch",
                                            heraldMsg("MessageSearch"));

  qmlRegisterUncreatableType<Conversations>("LibHerald", 1, 0, "Conversations",
                                            heraldMsg("Conversations"));

  // Provides access to per conversation content
  qmlRegisterSingletonType<ContentMap>(
      "LibHerald", 1, 0, "ContentMap",
      [](QQmlEngine* engine, QJSEngine* scriptEngine) {
        Q_UNUSED(scriptEngine)
        Q_UNUSED(engine)

        ContentMap* contentMap = new ContentMap();

        return contentMap;
      });


  // Wraps clipboard functionality
  qmlRegisterSingletonType<QQqmlClipBoard>(
      "LibHerald", 1, 0, "ClipBoard",
      [](QQmlEngine* engine, QJSEngine* scriptEngine) {
        Q_UNUSED(scriptEngine)
        Q_UNUSED(engine)
        QQqmlClipBoard* clipboard = new QQqmlClipBoard();

        return clipboard;
      });

  // per conversation content
  qmlRegisterUncreatableType<ConversationContent>(
      "LibHerald", 1, 0, "ConversationContent",
      errMsg("ConversationContent", "ContentMap"));

  qmlRegisterUncreatableType<Members>("LibHerald", 1, 0, "Members",
                                      errMsg("Members", "ConversationContent"));

  // messages and support types
  qmlRegisterUncreatableType<Messages>(
      "LibHerald", 1, 0, "Messages", errMsg("Messages", "ConversationContent"));

  qmlRegisterUncreatableType<MessageBuilder>(
      "LibHerald", 1, 0, "MessageBuilder",
      errMsg("MessagesBuilder", "Messages"));

  qmlRegisterUncreatableType<MediaAttachments>(
      "LibHerald", 1, 0, "MediaAttachments",
      errMsg("MediaAttachments", "MessageBuilder"));

  qmlRegisterUncreatableType<DocumentAttachments>(
      "LibHerald", 1, 0, "DocumentAttachments",
      errMsg("DocumentAttachments", "MessageBuilder"));

  qmlRegisterType<SharedConversations>("LibHerald", 1, 0,
                                       "SharedConversations");

  // Support model for emoji input
  qmlRegisterType<EmojiPicker>("LibHerald", 1, 0, "EmojiPicker");

  // bundle of constants used in the UI
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
