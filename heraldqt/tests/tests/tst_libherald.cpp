#include <QtTest>
#include <QSignalSpy>
#include <QProcess>
#include <QDebug>
#include <QDir>
#include "Bindings.h"

void kill_server(QProcess *server);

class LibHerald : public QObject
{
  Q_OBJECT

/*
 * The objects are very sensitive to the order they are initilized in
 * So here I just initialize pointers and alloc them later.
 * */
public:
  Config        *cfg        = nullptr;
  Conversations *convos     = nullptr;
  HeraldState   *h_state    = nullptr;
  HeraldUtils   *h_utils    = nullptr;
  Messages      *msgs       = nullptr;
  NetworkHandle *nwk_handle = nullptr;
  Users         *users      = nullptr;
  QProcess      *server     = nullptr;
  QThread       *bob_thread = nullptr;
  LibHerald*     bob        = nullptr;
  LibHerald(bool spawn_server_flag = true);
  ~LibHerald();

private slots:
// set up
  void initTestCase();
  void cleanupTestCase();
  // set up uniform config for receiving messages
  void messages_set_up();
  // destroy everything messages related
  void messages_tear_down();

  // makes a seperate instance of libherald to talk to over the spawned
  // loopback server
  void spawn_bob() {
    // create a new database for bob
    qputenv("HERALD_DB_PATH", "bob.sqlite3");
    // bobs parent is the thread. when the thread dies so does he.
    bob_thread = new QThread;
    bob        = new LibHerald(bob_thread);
    bob->moveToThread(bob_thread);
    connect(bob_thread, SIGNAL(started()), bob, SLOT(listen_for_messages()));
    bob_thread->start();
  }

  // called by bob on startup
  // just sits and spins waiting for messages
  void listen_for_messages() {}

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
LibHerald::LibHerald(bool spawn_server_flag)
{
  //clear db
  QFile file("store.sqlite3");
  file.remove();

  h_state = new HeraldState();
  // Bob never spawns a server, only Alice does, Alice is static.
  h_state->setConfigId(spawn_server_flag ? "Bob" : "Alice");
}

LibHerald::~LibHerald()
{
  if (server != nullptr)
     kill_server(server);
}

// spawns server in a pthread for the duration of the tests.
void LibHerald::initTestCase() {

  QString wd = "./../../../server";
  QString cargo = QDir::homePath() + "/.cargo/bin/cargo";
  QStringList build_args; build_args << "build";
  QStringList server_args; server_args << "run" << "--bin" << "stupid";
  QProcess cargo_build(this);

  cargo_build.setProcessChannelMode(QProcess::MergedChannels);
  cargo_build.setWorkingDirectory(wd);
  cargo_build.setProgram(cargo);
  cargo_build.setArguments(build_args);
  cargo_build.start();
  bool status = cargo_build.waitForFinished(-1);
  qDebug() << "cargo output:" << cargo_build.readAll();
  if (!status) {
    QFAIL("server failed to build");
  }

  server = new QProcess(this);
  server->setProcessChannelMode(QProcess::MergedChannels);
  server->setWorkingDirectory(wd);
  server->setProgram(cargo);
  server->setArguments(server_args);
  server->start();
  status = server->waitForStarted(-1);
  if (!status) {
    QFAIL("server failed to run");
  }
  this->thread()->sleep(1);
  qDebug() << "server start output: " << server->readAll();
}

void LibHerald::cleanupTestCase() {
  if (bob_thread != nullptr) bob_thread->quit();
  //get rid of env variabel
  qunsetenv("HERALD_DB_PATH");
  // remove bobs database
  QFile file("bob.sqlite3");
  file.remove();
}


void kill_server(QProcess *cargo_run) {
  if (cargo_run == nullptr) {
    qDebug("server process was null! Network Tests not accurate");
    return;
  }
  if (cargo_run->state() != QProcess::Running) {
    qDebug("server process was not running! Network Tests not accurate");
    return;
  }
  cargo_run->terminate();
}

/*
 *  CONFIG TEST CASES:
 *  these tests prove that config will not bork upon being created
 *  they require that heraldState already. which means they are
 *  unfortunately coupled to another set of functions.
**/
void LibHerald::test_config_set_name_data(){
  QTest::addColumn<QString>("name");

  QTest::newRow("standard case 1")  <<  "Nano Nacuno";
  QTest::newRow("standard case 2")  <<  "Frank Stoyvesson";
  QTest::newRow("naughty string 1") <<  "ЁЂЃЄЅІЇшщъыьэюя";
  QTest::newRow("naughty string 2") <<  "社會科學院語學研究所";
  QTest::newRow("naughty string 3") <<  "❤️ 💔 💌 💕 💞 💓 💗 💖 💘 💝 💟 💜 💛 💚 💙";

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


void LibHerald::test_config_set_color_scheme(){
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
void LibHerald::messages_set_up() {

  cfg = new Config();

  convos = new Conversations();

  while (convos->rowCount() > 0){
    convos->removeConversation(0);
  }

  auto bs = convos->addConversation();
  msgs = new Messages();
  msgs->setConversationId(bs);
}

void LibHerald::messages_tear_down() {

  while (convos->rowCount() > 0){
    convos->removeConversation(0);
  }

  delete cfg;
  delete convos;
  delete msgs;
}

void LibHerald::test_insertMessage() {
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

void LibHerald::test_deleteMessage() {
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

void LibHerald::test_reply() {
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
void LibHerald::test_modifyConversation() {
  convos = new Conversations;
  QSignalSpy data_changed_spy(convos, SIGNAL(dataChanged(QModelIndex, QModelIndex, QVector<int>)));

  // add some dummy conversations
  convos->addConversation();
  convos->addConversation();
  convos->addConversation();

  // these force changes to happen over (3,0) - (3,0)
  auto bs = convos->addConversation();

  convos->setColor(3, 100);
  auto changed_index = convos->index(3,0);
  auto args = data_changed_spy.at(0);

  QCOMPARE(data_changed_spy.count(), 1);
  QCOMPARE(args.at(0),changed_index);
  QCOMPARE(args.at(1),changed_index);
  QCOMPARE(convos->color(3), 100);


  convos->setTitle(3, "The Trapezoid Of Discovery");
  args = data_changed_spy.at(1);

  QCOMPARE(data_changed_spy.count(), 2);
  QCOMPARE(args.at(0),changed_index);
  QCOMPARE(args.at(1),changed_index);
  QCOMPARE(convos->title(3), "The Trapezoid Of Discovery");


  convos->setMuted(3, true);
  args = data_changed_spy.at(2);

  QCOMPARE(data_changed_spy.count(), 3);
  QCOMPARE(args.at(0),changed_index);
  QCOMPARE(args.at(1),changed_index);
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
  while (convos->rowCount() > 0 ){
    convos->removeConversation(0);
  }

  QCOMPARE(convos->rowCount(),0);
  delete convos;
}

// tests that need the server
void LibHerald::test_networkHandleConnects() {

  nwk_handle = new NetworkHandle();
  QSignalSpy net_up_spy(nwk_handle, SIGNAL(connectionUpChanged()));
  QSignalSpy net_pending_spy(nwk_handle, SIGNAL(connectionPendingChanged()));

  this->thread()->sleep(1);
  QCOMPARE(nwk_handle->connectionUp(), true);
  QCOMPARE(net_up_spy.count(), 2);
  QCOMPARE(net_pending_spy.count(), 2);

};

// void LibHerald::test_intraclientMessage() {

//  convos = new Conversations();
//  users = new Users;
//  msgs = new Messages();
//  this->thread()->sleep(1);

//  auto bs = convos->addConversation();
//  msgs->setConversationId(bs);
//  auto msg_bs = msgs->insertMessage("Hello Bob!");
//  QCOMPARE(bs.length(),32);
//  QCOMPARE(msg_bs.length(),32);
//  QCOMPARE(nwk_handle->sendAddRequest("Bob", bs), true);
//  QCOMPARE(nwk_handle->sendMessage("Hello Bob!", bs, msg_bs), true);

//  // todo: give bob a running spy and some methods to count how many
//  // messages he receives.

//};

QTEST_APPLESS_MAIN(LibHerald)

#include "tst_libherald.moc"
