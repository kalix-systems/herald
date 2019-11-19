#include "Bindings.h"
#include <QDebug>
#include <QSignalSpy>
#include <QtTest>

class libherald : public QObject {
  Q_OBJECT

public:
    Config*     cfg    = nullptr;
    HeraldState*  herald_state = nullptr;
    Conversations*  convos = nullptr;
    ConversationBuilder* convobuilder = nullptr;
    Messages*   msg = nullptr;
    MessageBuilder*  msgbuilder =  nullptr;
    Users*  user = nullptr;
    Errors* error = nullptr;
    libherald();
    ~libherald();

  private slots:
    void initTestCase();
    void test_config_set_name();
    void test_config_set_color();
    void test_convo_messages_setup();
    void test_convo_messages_deletion();
    void test_message_send_delete();
    void test_convo_settings();
    void test_convo_filter();
    void test_reply();
};

libherald::libherald() {}
libherald::~libherald() {}

void libherald::initTestCase()
{
  qDebug() << "Removing Previous Run Database";
  QFile file("db/store.sqlite3");
  file.remove();
  qDebug() << "Creating New Herald State";
  herald_state = new HeraldState();
  QSignalSpy spy(herald_state, SIGNAL(configInitChanged()));
  qDebug() << "Creating New Errors Queue";
  error = new Errors();
  qDebug() << "Registering New User 'Alice'";
  herald_state->registerNewUser("Alice");
  QVERIFY(spy.wait(1000));
}

void libherald::test_config_set_name()
{
  cfg = new Config();
  qDebug() << "Allocating New Config";
  QSignalSpy spy(cfg, SIGNAL(nameChanged()));
  qDebug() << "setting name in config";
  cfg->setName("Alice_Alias");
  QCOMPARE(cfg->name(), "Alice_Alias");
  QCOMPARE(spy.count(), 1);
  delete cfg;
}

void libherald::test_config_set_color()
{
  cfg = new Config();
  QSignalSpy spy(cfg, SIGNAL(colorChanged()));
  cfg->setColor(0);
  QCOMPARE(cfg->color(), 0);
  QCOMPARE(spy.count(), 1);
  delete cfg;
}

void libherald::test_convo_messages_setup() {
    cfg = new Config();
    convos = new Conversations();
    convobuilder = new ConversationBuilder();
    convobuilder -> finalize();
    auto cid = convos -> conversationId(0);
    msg = new Messages();
    QSignalSpy spy(msg, SIGNAL(conversationIdChanged()));
    msg -> setConversationId(cid);
    QCOMPARE(spy.count(), 1);
}

void libherald::test_convo_messages_deletion() {
    delete msg;
    for (qint64 i = 0; i < convos -> rowCount(); ++i) {
        if (convos -> conversationId(i) != cfg -> ntsConversationId()) {
            convos -> removeConversation(i);
        }
    }
    delete cfg;
    delete convos;
}

void libherald::test_message_send_delete() {
    test_convo_messages_setup();
    auto cid = msg -> conversationId();
    QSignalSpy spy(msg, &Messages::newDataReady);
    msgbuilder = new MessageBuilder();
    msgbuilder -> setConversationId(cid);
    msgbuilder -> setBody("test");
    msgbuilder -> finalize();
    QModelIndex testIndex = msg -> index(0,0);
    //enough time to receive signal
    std::this_thread::sleep_for (std::chrono::milliseconds(100));
    // this needs to happen bc reasons
    msg -> fetchMore(testIndex);
    QCOMPARE(msg -> rowCount(), 1);
    QCOMPARE(msg -> body(0), "test");
    //once also for data saved
    QCOMPARE(spy.count(), 2);

    msg -> deleteMessage(0);
    QCOMPARE(msg -> rowCount(), 0);
    test_convo_messages_deletion();
}

void libherald::test_convo_settings() {
    test_convo_messages_setup();
    convos -> setColor(0, 100);
    convos -> setTitle(0, "Nyah");
    convos -> setMuted(0, true);
    QCOMPARE(convos -> title(0), "Nyah");
    QCOMPARE(convos -> color(0), 100);
    QCOMPARE(convos -> muted(0), true);
    test_convo_messages_deletion();
}

void libherald::test_convo_filter() {
    cfg = new Config();
    convos = new Conversations();
    convobuilder = new ConversationBuilder();
    convobuilder -> finalize();
    convobuilder -> finalize();
    convos -> setTitle(0, "Bloom");
    convos -> setTitle(1, "wblob");
    convos -> setTitle (2, "Bla💖rgh");
    convos -> setFilter("Blo");
    QCOMPARE(convos -> matched(0), true);
    QCOMPARE(convos -> matched(1), true);
    QCOMPARE(convos -> matched(2), false);
    convos -> setFilter(".*");
    convos -> setFilterRegex(true);
    QCOMPARE(convos -> matched(0), true);
}

void libherald::test_reply() {
    test_convo_messages_setup();
    auto cid = msg -> conversationId();
    msgbuilder = new MessageBuilder();
    msgbuilder -> setConversationId(cid);
    msgbuilder -> setBody("test");
    msgbuilder -> finalize();
    QModelIndex testIndex = msg -> index(0,0);
    std::this_thread::sleep_for (std::chrono::milliseconds(100));
    msg -> fetchMore(testIndex);
    QCOMPARE(msg -> rowCount(), 1);

    auto msgId = msg -> messageId(0);
    msgbuilder -> setBody("reply");
    msgbuilder -> setReplyingTo(msgId);
    msgbuilder -> finalize();
    QModelIndex replyIndex = msg -> index(0, 1);
    std::this_thread::sleep_for (std::chrono::milliseconds(100));
    msg -> fetchMore(replyIndex);
    QCOMPARE(msg -> op(1), msgId);
    QCOMPARE(msg -> rowCount(), 2);

}

QTEST_MAIN(libherald)

#include "tst_libherald.moc"
