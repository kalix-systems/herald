#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <new>

namespace ffi {

static const int DATABASE_ERROR = 2;

static const int HERALD_ERROR = 1;

static const int MUTEX_ERROR = 3;

/// A constant buffer, templated over the `Item` type.
template<typename Item>
struct ConstBuffer {
  const Item *data;
  uintptr_t len;
};

extern "C" {

/// Frees a ConstBuffer.
void const_buffer_string_free(const ConstBuffer<const char*> *buf);

/// Returns number of items in a `ConstBuffer`
/// Returns -1 on failure.
int const_buffer_string_len(const ConstBuffer<const char*> *buf);

} // extern "C"

} // namespace ffi
