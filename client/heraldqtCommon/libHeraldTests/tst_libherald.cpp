#include "Bindings.h"
#include <QDebug>
#include <QDir>
#include <QProcess>
#include <QSignalSpy>
#include <QtTest>

class LibHerald : public QObject {
  Q_OBJECT

  /*
   * The objects are very sensitive to the order they are initilized in
   * So here I just initialize pointers and alloc them later.
   * */
public:
  Config*        cfg        = nullptr;
  Conversations* convos     = nullptr;
  HeraldState*   h_state    = nullptr;
  HeraldUtils*   h_utils    = nullptr;
  Messages*      msgs       = nullptr;
  NetworkHandle* nwk_handle = nullptr;
  Users*         users      = nullptr;
  LibHerald();
  ~LibHerald();

private slots:
  //  // set up
  void messages_set_up();
  // destroy everything messages related
  void messages_tear_down();

  // config test slots
  void test_config_set_name();
  void test_config_set_name_data();

  void test_config_set_color();
  void test_config_set_color_data();

  void test_config_set_color_scheme();
  void test_config_set_color_scheme_data();

  // conversation testing slot
  void test_modifyConversation();
  // message testing slots
  void test_insertMessage();
  void test_deleteMessage();
  void test_reply();
  // networking dependant tests
  void test_networkHandleConnects();
  //  void test_intraclientMessage();
};

/*
 * If this creation sequence aborts. you have failed test number 0.
 * */
LibHerald::LibHerald()
{
  // clear db
  QFile file("store.sqlite3");
  file.remove();

  h_state = new HeraldState();
  // Bob never spawns a server, only Alice does, Alice is static.
  h_state->setConfigId("Alice");
}

LibHerald::~LibHerald() {}

/*
 *  CONFIG TEST CASES:
 *  these tests prove that config will not bork upon being created
 *  they require that heraldState already. which means they are
 *  unfortunately coupled to another set of functions.
 **/
void LibHerald::test_config_set_name_data()
{
  QTest::addColumn<QString>("name");

  QTest::newRow("standard case 1") << "Nano Nacuno";
  QTest::newRow("standard case 2") << "Frank Stoyvesson";
  QTest::newRow("naughty string 1") << "ЁЂЃЄЅІЇшщъыьэюя";
  QTest::newRow("naughty string 2") << "社會科學院語學研究所";
  QTest::newRow("naughty string 3") << "❤️ 💔 💌 💕 💞 💓 💗 💖 💘 💝 💟 💜 💛 💚 "
                                       "💙";
}

void LibHerald::test_config_set_name()
{
  cfg = new Config();
  QSignalSpy spy(cfg, SIGNAL(nameChanged()));

  QFETCH(QString, name);

  cfg->setName(name);
  QCOMPARE(cfg->name(), name);
  QCOMPARE(spy.count(), 1);
  delete cfg;
}
void LibHerald::test_config_set_color_data()
{
  QTest::addColumn<quint32>("color");

  QTest::newRow("0") << 0u;
  QTest::newRow("1") << 1u;
  QTest::newRow("2") << 2u;
  QTest::newRow("3") << 3u;
  QTest::newRow("4") << 4u;
}
void LibHerald::test_config_set_color()
{
  cfg = new Config();
  QSignalSpy spy(cfg, SIGNAL(colorChanged()));

  QFETCH(quint32, color);

  cfg->setColor(color);
  QCOMPARE(cfg->color(), color);
  QCOMPARE(spy.count(), 1);
  delete cfg;
}

void LibHerald::test_config_set_color_scheme_data()
{
  QTest::addColumn<quint32>("color_scheme");

  QTest::newRow("0") << 0u;
  QTest::newRow("1") << 1u;
  QTest::newRow("2") << 2u;
  QTest::newRow("3") << 3u;
  QTest::newRow("4") << 4u;
}

void LibHerald::test_config_set_color_scheme()
{
  cfg = new Config();
  QSignalSpy spy(cfg, SIGNAL(colorschemeChanged()));

  QFETCH(quint32, color_scheme);

  cfg->setColorscheme(color_scheme);
  QCOMPARE(cfg->colorscheme(), color_scheme);
  QCOMPARE(spy.count(), 1);
  delete cfg;
}

/*
 *  MESSAGES TEST CASES:
 *  these are tests for the messages database.
 *  They do not rely on the server for operation.
 **/
void LibHerald::messages_set_up()
{

  cfg = new Config();

  convos = new Conversations();

  while (convos->rowCount() > 0) {
    convos->removeConversation(0);
  }

  auto bs = convos->addConversation();
  msgs    = new Messages();
  msgs->setConversationId(bs);
}

void LibHerald::messages_tear_down()
{

  while (convos->rowCount() > 0) {
    convos->removeConversation(0);
  }

  delete cfg;
  delete convos;
  delete msgs;
}

void LibHerald::test_insertMessage()
{
  messages_set_up();

  QSignalSpy spy(msgs, SIGNAL(rowsInserted(QModelIndex, int, int)));

  msgs->insertMessage("simple case 1");

  auto args = spy.at(0);
  QCOMPARE(spy.count(), 1);
  QCOMPARE(args.at(1), QVariant(0));
  QCOMPARE(args.at(2), QVariant(0));

  msgs->insertMessage("simple case 2");

  args = spy.at(1);
  QCOMPARE(spy.count(), 2);
  QCOMPARE(args.at(1), QVariant(1));
  QCOMPARE(args.at(2), QVariant(1));

  msgs->insertMessage("naughty string 社會科學院語學研究所");

  args = spy.at(2);
  QCOMPARE(spy.count(), 3);
  QCOMPARE(args.at(1), QVariant(2));
  QCOMPARE(args.at(2), QVariant(2));

  messages_tear_down();
}

void LibHerald::test_deleteMessage()
{
  messages_set_up();

  QSignalSpy spy(msgs, SIGNAL(rowsInserted(QModelIndex, int, int)));
  QSignalSpy rem_spy(msgs, SIGNAL(rowsRemoved(QModelIndex, int, int)));

  msgs->insertMessage("simple case 1");

  auto args = spy.at(0);
  QCOMPARE(spy.count(), 1);
  QCOMPARE(args.at(1), QVariant(0));
  QCOMPARE(args.at(2), QVariant(0));

  msgs->insertMessage("simple case 2");

  args = spy.at(1);
  QCOMPARE(spy.count(), 2);
  QCOMPARE(args.at(1), QVariant(1));
  QCOMPARE(args.at(2), QVariant(1));

  msgs->deleteMessage(1);

  args = rem_spy.at(0);
  QCOMPARE(rem_spy.count(), 1);
  QCOMPARE(args.at(1), QVariant(1));
  QCOMPARE(args.at(2), QVariant(1));

  msgs->deleteMessage(0);

  args = rem_spy.at(1);
  QCOMPARE(rem_spy.count(), 2);
  QCOMPARE(args.at(1), QVariant(0));
  QCOMPARE(args.at(2), QVariant(0));

  messages_tear_down();
}

void LibHerald::test_reply()
{
  messages_set_up();

  QSignalSpy spy(msgs, SIGNAL(rowsInserted(QModelIndex, int, int)));
  msgs->insertMessage("simple case 1");

  auto args = spy.at(0);
  QCOMPARE(spy.count(), 1);
  QCOMPARE(args.at(1), QVariant(0));
  QCOMPARE(args.at(2), QVariant(0));

  auto bs = msgs->insertMessage("simple case 2");

  args = spy.at(1);
  QCOMPARE(spy.count(), 2);
  QCOMPARE(args.at(1), QVariant(1));
  QCOMPARE(args.at(2), QVariant(1));

  msgs->reply("simple case 2 reply", bs);

  args = spy.at(2);
  QCOMPARE(spy.count(), 3);
  QCOMPARE(args.at(1), QVariant(2));
  QCOMPARE(args.at(2), QVariant(2));

  messages_tear_down();
}

/*
 *  CONVERSATION TEST CASE:
 *  these are tests for the messages database.
 *  They do not rely on the server for operation.
 **/
void LibHerald::test_modifyConversation()
{
  convos = new Conversations;
  QSignalSpy data_changed_spy(
      convos, SIGNAL(dataChanged(QModelIndex, QModelIndex, QVector<int>)));

  // add some dummy conversations
  convos->addConversation();
  convos->addConversation();
  convos->addConversation();

  // these force changes to happen over (3,0) - (3,0)
  auto bs = convos->addConversation();

  convos->setColor(3, 100);
  auto changed_index = convos->index(3, 0);
  auto args          = data_changed_spy.at(0);

  QCOMPARE(data_changed_spy.count(), 1);
  QCOMPARE(args.at(0), changed_index);
  QCOMPARE(args.at(1), changed_index);
  QCOMPARE(convos->color(3), 100u);

  convos->setTitle(3, "The Trapezoid Of Discovery");
  args = data_changed_spy.at(1);

  QCOMPARE(data_changed_spy.count(), 2);
  QCOMPARE(args.at(0), changed_index);
  QCOMPARE(args.at(1), changed_index);
  QCOMPARE(convos->title(3), "The Trapezoid Of Discovery");

  convos->setMuted(3, true);
  args = data_changed_spy.at(2);

  QCOMPARE(data_changed_spy.count(), 3);
  QCOMPARE(args.at(0), changed_index);
  QCOMPARE(args.at(1), changed_index);
  QCOMPARE(convos->muted(3), true);

  convos->setFilter("The Trap");
  args = data_changed_spy.at(3);

  QCOMPARE(data_changed_spy.count(), 4);
  QCOMPARE(convos->filter(), "The Trap");
  QCOMPARE(convos->matched(3), true);

  convos->setFilter(".*");
  convos->setFilterRegex(true);
  QCOMPARE(convos->matched(3), true);

  // dump convos from the DB
  while (convos->rowCount() > 0) {
    convos->removeConversation(0);
  }

  QCOMPARE(convos->rowCount(), 0);
  delete convos;
}

// tests that need the server
// should fail if the server is down
// and not at Loop back
void LibHerald::test_networkHandleConnects()
{
  QSKIP("Testing network handle skipped until automated server launch.");
  nwk_handle = new NetworkHandle();
  QSignalSpy net_up_spy(nwk_handle, SIGNAL(connectionUpChanged()));
  QSignalSpy net_pending_spy(nwk_handle, SIGNAL(connectionPendingChanged()));

  this->thread()->sleep(1);
  QCOMPARE(nwk_handle->connectionUp(), true);
  QCOMPARE(net_up_spy.count(), 2);
  QCOMPARE(net_pending_spy.count(), 2);
  delete nwk_handle;
};

QTEST_APPLESS_MAIN(LibHerald)

#include "tst_libherald.moc"