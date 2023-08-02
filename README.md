# Mapage

Mapage is a type-namespaced key value storage system which uses [async-graphql](https://crates.io/crates/async-graphql) for client interaction.

Basically what is meant by "type-namespaced" is each storable type gets its own hashmap and its own part of the API.

<br/>

## Why design Mapage this way?

Mapage originated as a Redis-like system, but when I develved into Any trait references I decided to re-orient the project around hashmaps instead.

I did this because it seemed to me that this would probably be more space-efficient as you wouldn't have to have an Any pointer for each object stored (the size of at least two 64 bit integers!) and time-efficient because you wouldn't have to figure out what type an object was every time you wanted to do something with it (This may be the case with enums as well).

This also means that incorrect type errors can mostly be eliminated.

<br/>

## Atomicity Levels

The namespaced-type design makes it reasonably straightforward to support different atomicity levels:

| Level         | Description |
| -----------   | ----------- |
| Store         | Only one action (read or write) can be performed on the store per interaction. |
| Namespace     | Only one action can be performed on each individual namespace per interaction. |
| Sub-Namespace | A section of the namespace is locked per interaction (buckets) |
| Value         | Each value is interacted with by each thread on an individual level and the namespace is seldom if ever locked | 

And you can have other combinations where you can retrieve an object for interacting with indivdual objects under the top three above levels:

| Level         |
| -----------   |
| Store-Value   |
| Namespace-Value   |
| Sub-Namespace-Value |

But we won't be focusing on these right now (or probably for a while...).

In terms of features atomicity levels are divided into two broad categories:

| Store level Feature |
| -----------   |
| store_aml     | 
| sub_store_aml |

With store_aml everything must be interacted with using a single means (e.g a Mutex), but with sub_store_aml you have more discretion about how each namespace goes about handling interaction.

Currently only sub_store_aml related features have been implemented: 

| Default Storage Implemtation Feature |
| -----------   |
| scc_hashmap_namespaces |
| dashmap_namespaces |

These are the namespace implementations that will be used by default for each type-feature specified as part of the --features argument to cargo.

Only specify one.

<br/>

## The Types

| Type Feature  | Description |
| -----------   | ----------- |
| all_types     | Everything  |
| char          |             |
| f32           |             |
| f64           |             |
| i8            |             |
| i16           |             |
| i32           |             |
| i64           |             |
| i128          | An async-graphql scalar value |
| isize         |             |
| String        |             |
| u8            |             |
| u16           |             |
| u32           |             |
| u64           |             |
| u128          | An async-graphql scalar value |
| usize         |             |
| Whatever      | A GraphQL union of evrything except SelectedType |
| SelectedType  | A GraphQL union of whatever else was selected except Whatever |

As you can see if you want everything specifiy all_types when providing features to cargo. When you specify a type a namespace is added to Mapage, this is implemented as specified when you provided the default storage implemtation feature with a part of GraphQL API generated for that type. For instance if you specify an i32 type you get accessor methods to the map that your i32s would be stored in and operator methods for manipulating values as well. With String and other methods mutation methods other that for value replacement are not yet supported.

<br/>

Specify the Whatever type if you want to to be able store whatever types Mapage supports and async-graphql allows in a single value. As Whatever is a GraphQL union type it cannot not store other unions i.e SelectedType.
SelectedTypes store whatever other types the user has specified (except Whatevers obviously as that is also a GraphQL union).

<br/>

## SelectedTypeIO

The SelectedTypeIO feature adds GraphQL methods which enable users to query mutliple namespaces asynchronously. They allow (or should allow) only one namespace query per request. What the user needs to do is provide the keys to look up and the namespaces (type names) in which the values are looked up and added to the results Vec.

You might use this method if you want to store the fields of an object on a Mapage server but want to have it so each field can be mutated with out having to rediscover its type. This storege scheme also allows you to share fields between querys as well.

<br/>

## Usage Considerations

Mapage is intended for the out-of-process storage of data. Out-of-process storage meaning application data being put in a separate process or processes from where the main work is being done.

This is benefical because the data is isolated from what is happening in other applications and this data can be easily read and mutated by these applications (if they're on the same network).

Some downsides to using Mapage, when compared to in-process caching, might be the costs of serialisation and deserialisation and network communications.

<br/>

Though the downside to in-process caching is that it is vulerable to whatever ever other broad application affecting events occur (panics, power-loss etc).

<br/>

Basically you'd want to use Mapage when you want a dedicated data cache that is isolated from other processes in your system setup, but not necessarily a persistent or authoritative storage location.

<br/>

## Over-Development

The primary goal of Mapage is to provide features that have a level of performance that users will find acceptable at minimum.

The secondy goal is to provide features that have either questionable performance characteristics or clearly bad performance characteristics as I think it would be both intersting and useful to be able to compare and contrast how different syncronistation methodologies affect how long it takes the system to get things done.  

For instance how would a Tokio Mutex enforcing store level atomicity on all its namespaces compare against SCC Hashmaps enforcing sub-namespace level atomicity on their namespace data?

A type-namespaced cache such as Mapage should make implementing both approaches fairly straight-forward so this kind of question can be answered.

<br/>

## Todo:

- Add store management features which would handle:
    - Object evictions.
    - Authentication and authorisation
    - Persistence
    - Replication (partial and whole)
    - Events (probably required by replication)
- Add collections types such as Standard Vec and Hashmap.
- Add immutable namespaces, where the values can be added, retrieved, removed, replaced, have only immutable methods called on them. Also this is where collections are put in Arcs (sub-namespace and value atomicity levels only probably).
- Async-graphql response caching (possibly in 5 second increments)
- Scripting with Rhai and Lua
- Arc'd String Keys (feature - best used in conjection with store management features)
- Add error conditions and messages for conflicting feature specifications.
- Vector searching and comparison.
- Add more namespace implementations using collection types like [Moka](https://crates.io/crates/moka) and std::Hashmap etc.
- Complete namespace implementations for the store and atomicity levels.
- Clean up the code.
- Document the code. 

## Possible:

- VRAM storege
- Option Types (Unlikely, unless it's Vec\<Option\<T\>\>)
- Non-String keys - integer types
- Add support for gRPC (probably using [Tonic](https://crates.io/crates/tonic))

<br/>

## About Generic Permutations

In order to control the amount of work requred certain classes of type will or won't be implemented:

- No option unless in other gernerics e.g. Vec\<Option\<T\>\>. While having a base level Option could be a convenient way to reserve a key in a namespace, for the time being at least, base level Option namespaces wont be added.
- With hashmaps and other generic types that take multiple auguments you can have all manner of different combinations. So namespace hashmap implementations will probably be constrained to String or whatever other type is desired for the key and then the value types will be the other scalar and collection values offered by Mapage (sub-namespaces basically).
- Other generic collections such as: std::BTreeMap and std::HashSet may eventually be supported.

<br/>

This project has come along way since its inception in mid-2022 as a Redis-like cache and it has a long way to go still. Please use with the understanding that this project is a work in progress,

<br/>

## Coding Style

The coding style of this project emphasises the use of white space over keeping the line and column counts low.

So this:

```rust
fn foo()
{

    bar();

}

```

Not this:

```rust
fn foo()
{
    bar();
}

```

<br/>

## License

Licensed under either of:

- Apache License, Version 2.0, ([LICENSE-APACHE](./LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0 (see also: https://www.tldrlegal.com/license/apache-license-2-0-apache-2-0))
- MIT license ([LICENSE-MIT](./LICENSE-MIT) or http://opensource.org/licenses/MIT (see also: https://www.tldrlegal.com/license/mit-license))

at your discretion

<br/>

## Contributing

Please clone the repository and create an issue explaining what feature or features you'd like to add or bug or bugs you'd like to fix and perhaps how you intend to implement these additions or fixes. Try to include details though it doesn't need to be exhaustive and we'll take it from there (dependant on availability).

<br/>

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

