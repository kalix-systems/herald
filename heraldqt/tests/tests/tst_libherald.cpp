#include <QtTest>
#include <QSignalSpy>
#include <pthread.h>
#include "Bindings.h"

// spawns server in a pthread for the duration of the tests.
pid_t spawn_server() {
  // build server, wait.
  // spawn server in thread, return pid or -1
}

// kills the server at process ID pid,
// returns 0 on sucess, otherwise an error code
int kill_server(pid_t pid) {

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
  pid_t server_pid;
  LibHerald();
  ~LibHerald();
  void messages_set_up();
  void messages_tear_down();

private slots:
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
  void test_clearConversationView();
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
LibHerald::LibHerald()
{
  h_state = new HeraldState();
  h_state->setConfigId("Alice");
  server_pid = spawn_server();
}

LibHerald::~LibHerald()
{
  kill_server(server_pid);
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
}

void LibHerald::messages_tear_down() {

}
void LibHerald::test_insertMessage() {}
void LibHerald::test_clearConversationView() {}
void LibHerald::test_deleteConversation() {}
void LibHerald::test_deleteConversationById() {}
void LibHerald::test_deleteMessage() {}
void LibHerald::test_refresh() {}
void LibHerald::test_reply() {}

// network dependant tests
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

QTEST_APPLESS_MAIN(LibHerald)

#include "tst_libherald.moc"
