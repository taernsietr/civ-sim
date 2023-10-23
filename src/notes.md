Aspen — Today at 9:40 PM
I'd probably use an mpsc channel where each worker thread has a
 sender and the main thread has the receiver. a worker thread
 sends its unique ID (determining the ordering) along with the
 struct via the channel when it's done, and the main thread puts
 the struct into the vec in the appropriate slot determined by
 the ID

Aspen — Today at 9:41 PM
you can use an atomic counter to sequentially generate unique 
IDs for the threads btw, just have each thread fetch_add(1, 
Ordering::Relaxed) from a static AtomicUsize on startup (edited)

impl Fox for Salix — Today at 9:43 PM
Using scoped threads and if your struct implements Default, you
 could create a vec of the required size and hand out a mutable
 ref to each index using split_first repeatedly

impl Fox for Salix — Today at 9:44 PM
If your struct does not implement default, you could use 
Vec<Option<_>> and then map and collect back into Vec<_>
And if you can't use scoped threads, then use channels 

Aspen — Today at 9:44 PM
btw if the number of structs is large, you might want to use a
 thread pool instead of one thread per struct
