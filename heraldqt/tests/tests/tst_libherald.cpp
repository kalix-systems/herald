#include <QtTest>
#include <QSignalSpy>
#include <pthread.h>
#include "Bindings.h"
// spawns server in a pthread for the duration of the tests.
pid_t spawn_server() {

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

private slots:
  void test_config_set();
  void test_messages_insertion();


};

/*
 * If this creation sequence aborts. you have failed test number 0.
 * */
LibHerald::LibHerald()
{
  h_state = new HeraldState();
  h_state->setConfigId("Alice");
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

/*
 * arrangement: all properties of the config are set
 * methods : insert_message, get_message,
 * expected behavior:
 *      set properties should emit data changed
 *      retrieved properties should emit nothing
 *      the retrieved values should be identical
 *      to the stored ones.
**/
void LibHerald::test_config_set()
{

}

/*
 *  MESSAGES TEST CASES:
 *  these are tests for the messages database.
 *  They do not rely on the server for operation.
**/

/*
 * arrangement: messages are inserted into the database
 * methods : insert_message, get_message,
 * expected behavior:
 *      inserted messages should emit data changed
 *      retrieved mesages should emit nothing
 *      the retrieved messages should be identical
 *      to the inserted messages.
**/
void LibHerald::test_messages_insertion()
{

}

QTEST_APPLESS_MAIN(LibHerald)

#include "tst_libherald.moc"
