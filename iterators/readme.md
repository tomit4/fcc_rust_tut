## Iterator

- An Iterator allows you to perform a task on a <b>sequence</b> of items <b>in
  turn</b>

- Iterators are <b>lazy</b>, meaning they have <b>no effect<b> until methods are
  called that <b>consume</b> the iterator to use it up

- All iterators implement the trait <b>Iterator</b>, which has a `next()`
  method. The `next()` method gets called automatically when traversing over some
  data

- Some methods <b>consume</b> the iterator while others <b>produce a new
  iterator</b> from the provided iterator
