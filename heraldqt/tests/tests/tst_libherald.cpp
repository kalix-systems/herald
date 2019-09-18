#include <QtTest>
#include <QSignalSpy>
#include <QProcess>
#include "Bindings.h"

// spawns server in a pthread for the duration of the tests.
void spawn_server(QProcess *cargo_run) {
  // build server, wait.
  // spawn server in thread, return pid or -1
  QString cargo = "cargo";
  QStringList build_args = { "build", "--c", "some args i need to look up"};
  QStringList server_args = { "run", "--bin", "stupid"};

  QProcess cargo_build;
  cargo_build.start(cargo, build_args);
  cargo_build.waitForFinished(300000);

  cargo_run = new QProcess;
  cargo_run->setProgram(cargo);
  cargo_run->setArguments(build_args);
  cargo_run->start();
}

// kills the server at process ID pid,
// returns 0 on sucess, otherwise an error code
int kill_server(QProcess *cargo_run) {
  cargo_run->kill();
}

// add necessary includes here

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
  QProcess *server = nullptr;
  LibHerald(bool spawn_server_flag = true);
  ~LibHerald();
  void messages_set_up();
  void messages_tear_down();

private slots:
// config test slots
  void test_config_set_name();
  void test_config_set_name_data();

  void test_config_set_color();
  void test_config_set_color_data();

  void test_config_set_pfp();
  void test_config_set_pfp_data();

  void test_config_set_color_scheme();
  void test_config_set_color_scheme_data();

// conversation testing slots
  void test_filter();
  void test_setFilter();
  void test_filterRegex();
  void test_setFilterRegex();
  void test_addConversation();
  void test_removeConversation();
  void test_toggleFilterRegex();
// message testing slots
  void test_insertMessage();
  void test_deleteConversation();
  void test_deleteConversationById();
  void test_deleteMessage();
  void test_refresh();
  void test_reply();
// networking dependant tests

};

/*
 * If this creation sequence aborts. you have failed test number 0.
 * */
LibHerald::LibHerald(bool spawn_server_flag)
{
  h_state = new HeraldState();
  h_state->setConfigId("Alice");

  if (spawn_server_flag)
        spawn_server(server);
}

LibHerald::~LibHerald()
{
  if (server != nullptr)
    kill_server(server);
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

void LibHerald::test_config_set_pfp_data(){
  QTest::addColumn<QString>("url");
  QTest::newRow("standard case 1")  <<  "NanoNacuno.png";
  QTest::newRow("standard case 2")  <<  "FrankStoyvesson.png";
  QTest::newRow("naughty string 1") <<  "ЁЂЃЄЅІЇшщъыьэюя.jpeg";
  QTest::newRow("naughty string 2") <<  "社會科學院語學研究所.jpg";
  QTest::newRow("naughty string 3") <<  "❤️ 💔 💌 💕 💞 💓 💗 💖 💘 💝 💟 💜 💛 💚 💙.png";
}


void LibHerald::test_config_set_pfp()
{
  cfg = new Config();
  QSignalSpy spy(cfg, SIGNAL(profilePictureChanged()));

  QFETCH(QString, url);

  cfg->setProfilePicture(url);
  QCOMPARE(cfg->profilePicture(), ""); // all of these should fail none of the paths exist
  QCOMPARE(spy.count(), 0);
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

  while (convos->rowCount() > 0 ){
    convos->removeConversation(0);
  }

  auto bs = convos->addConversation();
  msgs = new Messages();
  msgs->setConversationId(bs);
}

void LibHerald::messages_tear_down() {

  while (convos->rowCount() > 0 ){
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


void LibHerald::test_deleteConversation() {


}

void LibHerald::test_deleteConversationById() {}
void LibHerald::test_refresh() {}
void LibHerald::test_reply() {}

/*
 *  CONVERSATION TEST CASES:
 *  these are tests for the messages database.
 *  They do not rely on the server for operation.
**/
void LibHerald::test_filter() {}
void LibHerald::test_setFilter() {}
void LibHerald::test_filterRegex() {}
void LibHerald::test_setFilterRegex() {}
void LibHerald::test_addConversation() {}
void LibHerald::test_removeConversation() {}
void LibHerald::test_toggleFilterRegex() {}

// tests that need the server

QTEST_APPLESS_MAIN(LibHerald)

#include "tst_libherald.moc"
