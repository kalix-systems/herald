#include "Bindings.h"
#include <QtTest>
#include <QSignalSpy>
#include <QDebug>
// add necessary includes here

class libherald : public QObject
{
  Q_OBJECT

public:
    Config*     cfg    = nullptr;
    HeraldState*  herald_state = nullptr;
    Conversations*  convos = nullptr;
    ConversationBuilder* convobuilder = nullptr;
    Messages*   msg = nullptr;
    MessageBuilder*  msgbuilder =  nullptr;
    Errors* error = nullptr;
    libherald();
    ~libherald();

private slots:
  void test_config_set_name();
  void test_config_set_color();
  void test_convo_messages_setup();
  void test_convo_messages_deletion();
  void test_message_send();

};

libherald::libherald()
{
    QFile file("db/store.sqlite3");
    file.remove();
    herald_state = new HeraldState();
    herald_state -> registerNewUser("Boruto");
    herald_state -> setConfigInit(true);
    error = new Errors();

}

libherald::~libherald() {}

void libherald::test_config_set_name()
{
    cfg = new Config();
    QSignalSpy spy(cfg, SIGNAL(nameChanged()));
    cfg -> setName("Alfalfa");
    QCOMPARE(cfg -> name(), "Alfalfa");
    QCOMPARE(spy.count(), 1);
    delete cfg;
}

void libherald::test_config_set_color() {
    cfg = new Config();
    QSignalSpy spy(cfg, SIGNAL(colorChanged()));
    cfg -> setColor(0);
    QCOMPARE(cfg -> color(), 0);
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

void libherald::test_message_send() {
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
    //once also for data saved
    QCOMPARE(spy.count(), 2);
    test_convo_messages_deletion();
}

QTEST_APPLESS_MAIN(libherald)

#include "tst_libherald.moc"
