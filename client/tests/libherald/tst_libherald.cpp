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
    libherald();
    ~libherald();

private slots:
  void test_config_set_name();
  void test_config_set_color();
  void test_convo_messages_setup();
  void test_convo_messages_deletion();

};

libherald::libherald()
{
    QFile file("db/store.sqlite3");
    file.remove();
    herald_state = new HeraldState();
    herald_state -> registerNewUser("Boruto");
    herald_state -> setConfigInit(true);

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
    qDebug() << "delete message";
    while (convos -> rowCount() > 0) {
        qDebug() << "pre loop";
        convos -> removeConversation(0);
        qDebug() << "after loop";
    }
    qDebug() << "delete convos loop";
    delete cfg;
    qDebug() << "delete config";
    delete convos;
    qDebug() << "delete convos";
}

QTEST_APPLESS_MAIN(libherald)

#include "tst_libherald.moc"
