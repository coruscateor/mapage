Atomicity Levels (features):

store

sub_store atomicity levels:

namespace

sub_namespace

value

Other Possible Combinations:

store_value

namespace_value

sub_namespace_value


store_aml or sub_store_aml

If store_aml is specified as a feature then at least one of types and a sync type and a non-sync hashmap must be specified also, the instance of sync type will posess all user data hashmaps.

If sub_store_aml is specified as a feature then if one or more of the types is specified then a sync type and a non-sync hashmap or a sync hashmap must also be specified and this with be for all specified user types.
Depending on how user types were specified exceptions or additions can be made that deviate from the default types if any were specified.

store features:

user type:

all_types
char
f32
f64
i8
i16
i32
i64
i128
isize
string
u8
u16
u32
u64
u128
unit_value
usize
whatever

collections...

sync-method:

mutex
rwLock
actrusty_actor
async_mutex
async_rwLock

sync type:

std_mutex
std_rwlock
crossbeam_shardedlock (rwlock like)
parking_lot_fairmutex
parking_lot_mutex
parking_lot_rwlock
tokio_mutex
tokio_rwlock

Actrusty_actors (sync-types)

st_io_actor
ts_io_actor
ttsb_io_actor

map types:

std_hashmap         (store & sub_store - namespace level)
//hashbrown_hashmap   (store & sub_store - namespace level)
scc_hashmap         (sync - sub_namespace level)
scc_hashindex       (sync - sub_namespace level)
scc_treeindex       (sync - sub_namespace level)
dashmap             (sync - sub_namespace level)

sub_namespace hashmaps:

scc_hashmap
dashmap
scc_hashindex
scc_treeindex

sets:

std_hashset         (non-sync)
hashbrown_hashset   (non-sync)
scc_hashset         (sync - value level)
dashset             (sync - value level)


sub_store features:

Exeptions to sepecified defaults or additions to what has been specified can be added as follows:

<sync_type or map type>_<user type>

If the map type is not sync the sync type for the user type will also need to be specified if it isn't cover by the default specification.

Value Type Features:

Default Actor Value Types:

<actor type>_value

User Type Specific Actor Value Types:

<actor type>_<user type>_value

Arcs:

arc_value

arc_<user type>_value

Applicable only to collections and atomics

When used with collections creates a sitiaution where the collection is stored in the arc and replaced when it needs to be mutated.

Atomics:

atomic_value

atomic_<user type>_value

Applicable only to user types which have std atomic counterparts


sync type values:

<sync type>_value

User Type Specific Actor Value Types:

<sync type>_<user type>_value
