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

private slots:
  void test_config_set_name();
  void test_config_set_color();
  void test_config_set_pfp();
  void test_config_set_color_scheme();
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



/* run input tests single output
 *
 * for generically running tests that return one value
 * as opposed to a vector or map.
 *
 * T: the tested class
 * I: the input type
 * O: the output type, often the same as I
 *
 * these tests assume that input must match output.
 * and that all signals must be emitted. they are super generic.
 *
 * platform: The object to init and test
 * input: the input, and expected output, vectors
 * setter: the setter function pointer
 * getter: the getter function pointer
 * signal: a signal that should be listened for and spied on after each insertion
 *
 * */
template <class T, typename I, typename O>
void run_input_tests_single_output
(T *platform, std::vector<O> input, void(T::*setter)(I),  O(T::*getter)() const, const char* sig_name)
{
  platform = new T;
  QSignalSpy spy(platform, sig_name);
  int ct = 0;
  for (auto input_row : input) {
    ct++;
    (*platform.*setter)(input_row);
    QCOMPARE((*platform.*getter)(),input_row);
    QCOMPARE(spy.count(), ct);
  }
  delete platform;
}

/*
 * identical to the above function, except it checks if the vector output is
 * the same as the vector input.
 * */
template <class T, typename I, typename O>
void run_input_tests_vector_output
(T *platform, std::vector<O> input, void(T::*setter)(I),  O(T::*getter)() const, const char* sig_name)
{
  platform = new T;
  QSignalSpy spy(platform, sig_name);
  int ct = 0;
  for (auto input_row : input) {
    ct++;
    (*platform.*setter)(input_row);
    QCOMPARE((*platform.*getter)(),input_row);
    QCOMPARE(spy.count(), ct);
  }
  delete platform;
}


/*
 *  CONFIG TEST CASES:
 *  these tests prove that config will not bork upon being created
 *  they require that heraldState already. which means they are
 *  unfortunately coupled to another set of functions.
**/

void LibHerald::test_config_set_name()
{
  typedef const QString& input_t;
  typedef QString output_t;

  // cannot pass double reference type
  std::vector<output_t> input = {"Nano Nacuno", "Frank Stoyvesson", "ЁЂЃЄЅІЇшщъыьэюя"};
  run_input_tests_single_output<Config, input_t, output_t>(cfg,
                                  input,
                                  &Config::setName,
                                  &Config::name,
                                  SIGNAL(nameChanged()));

}

void LibHerald::test_config_set_color()
{
  typedef quint32 input_t;
  typedef quint32 output_t;
  std::vector<input_t> input = {0, 1, 2, 3, 4, 0, 1, 2, 3, 4};
  run_input_tests_single_output<Config, input_t, output_t>(cfg,
                                   input,
                                   &Config::setColor,
                                   &Config::color,
                                   SIGNAL(colorChanged()));

}

void LibHerald::test_config_set_pfp()
{
  typedef const QString& input_t;
  typedef QString output_t;

  // cannot pass double reference type. grr.
  std::vector<output_t> input = {"some_pfp.url", "some_other_pfp.url", "ЁЂЃЄЅІЇшщъыьэюя"};
  run_input_tests_single_output<Config, input_t, output_t>(cfg,
                                             input,
                                             &Config::setProfilePicture,
                                             &Config::profilePicture,
                                             SIGNAL(profilePictureChanged()));
}


void LibHerald::test_config_set_color_scheme(){

  typedef quint32 input_t;
  typedef quint32 output_t;

  std::vector<input_t> input = {0, 1, 2, 3, 4, 0, 1, 2, 3, 4};
  run_input_tests_single_output<Config, input_t, output_t>(cfg,
                                             input,
                                             &Config::setColorscheme,
                                             &Config::colorscheme,
                                             SIGNAL(colorschemeChanged()));

}



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
/*
 *  MESSAGES TEST CASES:
 *  these are tests for the messages database.
 *  They do not rely on the server for operation.
**/
void LibHerald::test_insertMessage() {}
void LibHerald::test_clearConversationView() {}
void LibHerald::test_deleteConversation() {}
void LibHerald::test_deleteConversationById() {}
void LibHerald::test_deleteMessage() {}
void LibHerald::test_refresh() {}
void LibHerald::test_reply() {}

QTEST_APPLESS_MAIN(LibHerald)

#include "tst_libherald.moc"
