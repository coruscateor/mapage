# Mapage

Mapage is a type-namespaced key-value storage system which uses [async-graphql](https://crates.io/crates/async-graphql) for client interaction.

Basically what is meant by "type-namespaced" is each storable type gets its own hashmap and its own part of the API.

<br/>

## Design Goals

- Customisable: Choose which types you want to store and which map implementations you want store them in.
- Easily-Optimisable: Easily find the best features for the fastest results.
- Type-Namespaced: Incorrect type errors should be few and far between, or non-esitant when you specify exacty which types you want stored.
- file storage: Chache files with ease.
- Persistable: Store any changes to your chached data in a database system of your choice or on the file .
- Scriptable: Implement the funtionality of your choice.
- Monitorable: Easily find out things like CPU and RAM usage.
- Searchable: From rudimentary to regex to vector based searching functionality.

<br/>

## Atomicity Levels

The namespaced-type design makes it reasonably straightforward to support different atomicity levels:

| Level         | Description |
| -----------   | ----------- |
| Store         | Only one action (read or write) can be performed on the store per interaction. |
| Namespace     | Only one action can be performed on each individual namespace per interaction. |
| Sub-Namespace | A section of the namespace is locked per interaction (buckets) |
| Value         | Each value is interacted with by each thread on an individual level and the namespace is seldom if ever locked | 

In terms of features atomicity levels are divided into two broad categories:

| Store level Feature |
| -----------   |
| store_aml     | 
| sub_store_aml |

With store_aml (store atomicity level) all the data is contained in a single synchronisation object e.g a Mutex or an Actor, but with the sub_store_aml (sub-store atomicity level) feature you have more discretion about how each namespace goes about handling user interaction.

store_aml is not implemented.

Once you've selected sub_store_aml you must select a default sore implementation feature: 

| Default Storage Implemtation Feature |
| -----------   |
| scc_hashmap_namespaces |
| dashmap_namespaces |

As you can probably sermise; when you select "scc_hashmap_namespaces" you get scc::Hashmap namespace implementations by default and when you select "dashmap_namespaces" you get Dashmap namespace implementations by default.

Select only one of these.

<br/>

## All Features

| Feature                   | Description |
| -----------               | ----------- |
| store_aml                 | Store atomicity level |   
| sub_store_aml             | Sub-store atomicity level |
| scc_hashmap_namespaces    | Use scc::Hashmap namespace implementations by default |
| dashmap_namespaces        | Use Dashmap namespace implementations by default |
| all_types                 | All types   |
| char                      | char type   |
| f32                       | f32 type    |
| f64                       | f64 type    |
| i8                        | i8 type     |
| i16                       | i16 type    |
| i32                       | i32 type    |
| i64                       | Broken - incompatible with GraphQL/async-graphql |
| i128                      | Broken - incompatible with GraphQL/async-graphql |
| isize                     | To be removed |
| String                    | String type |
| u8                        | u8 type     |
| u16                       | u16 type    |
| u32                       | u32 type    |
| u64                       | Broken - incompatible with GraphQL/async-graphql |
| u128                      | Broken - incompatible with GraphQL/async-graphql |
| usize                     | To be removed |
| Whatever                  | A GraphQL union of everything except SelectedType |
| SelectedType              | A GraphQL union of whatever else was selected except Whatever - To be removed |
| Vec_bool                  | A std::vec::Vec of bools |
| Vec_char                  | &nbsp;&nbsp;&nbsp;&nbsp; " &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; chars |
| Vec_f32                   | &nbsp;&nbsp;&nbsp;&nbsp; " &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; f32s  |
| Vec_f64                   | &nbsp;&nbsp;&nbsp;&nbsp; " &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; f64s  |
| Vec_i8                    | &nbsp;&nbsp;&nbsp;&nbsp; " &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; i8s   |
| Vec_i16                   | &nbsp;&nbsp;&nbsp;&nbsp; " &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; i16s  |
| Vec_i32                   | &nbsp;&nbsp;&nbsp;&nbsp; " &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; i32s  |
| Vec_i64                   | Broken - incompatible with GraphQL/async-graphql |
| Vec_i128                  | Broken - incompatible with GraphQL/async-graphql |
| Vec_isize                 | To be removed |
| VecSelectedType           | To be removed |
| VecString                 | A std::vec::Vec of std::string::Strings |
| Vec_u8                    | &nbsp;&nbsp;&nbsp;&nbsp; " &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; u8s  |
| Vec_u16                   | &nbsp;&nbsp;&nbsp;&nbsp; " &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; u16s |
| Vec_u32                   | &nbsp;&nbsp;&nbsp;&nbsp; " &nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp; u32s |
| Vec_u64                   | Broken - incompatible with GraphQL/async-graphql |
| Vec_u128                  | Broken - incompatible with GraphQL/async-graphql |
| Vec_usize                 | To be removed           |
| VecWhatever               | A std::vec::Vec of Whatevers |
| all_key_types_String      | Use std::string::String keys by default |
| all_key_types_Arc_String  | Use std::sync::Arc containing std::string::String keys by default |

<br/>

## Settings

To be implemented

<br/>

## Build Examples

One with everything:

```
cargo build --features=sub_store_aml,scc_hashmap_namespaces,all_types,all_key_types_String
```

In the above build you have the "sub_store_aml" feature to indicate that each namespace handles synchronisation independently.

The "scc_hashmap_namespaces" feature indicates that you want the namespace implemtations to be scc::Hashmaps by default.

The "all_types" feature indicates that you want all the types that mappage supports to be inculded in the build.

Finally the "all_key_types_String" feature indicates that you want the keys for all the included types to be std::string::Strings by default.

<br/>

Using the all_types feature, while convenient, will produce a mapage application with gigantic GraphQL API.

Why don't we try and be more a bit more selective:

```
cargo build --features=sub_store_aml,dashmap_namespaces,bool,char,i32,Whatever,all_key_types_String
```

Here instead of sub_store_aml, we have dashmap_namespaces for the namespace implementations.

And instead of all_types there are four individual types specified: bool, char, i32 and Whatever.

<br/>

When you know what you want to store you'll avoid using space unnecessary and have a more consise API than you would've had selecting everything available.

<br/>

## Build Must-Haves

To be explicit, in every build you need or should have specified:

1. The store atomicity level (only sub_store_aml is valid right now)
2. The namespace implementations for each selected type to use by default.
3. The types you want (char, f32, i8 etc) (this is optional but you wont have much of a cache if you skip this step)
4. The key type to use for each namespace by default

Feature incompatabilty error conditions have not be implemented yet so you may have to do some reading if you get something wrong. 

## Optimal Builds

To be implemented

<br/>

## A note on experimentation

While you would implement the most optimal features for practical usage scenarios, sub-optimal features will also be added for curiositys sake.

<br/>

## Todo:

- Add store/namespace management features which would handle:
    - Object evictions.
    - Authentication and authorisation
    - Persistence
    - Replication (partial and whole)
    - Events/Streaming
- Add sub-namespaces
- Add immutable namespaces, where the values can be added, retrieved, removed, replaced, have only immutable methods called on them. Also this is where collections are put in Arcs.
- Async-graphql response caching (possibly in 5 second increments)
- Scripting with Rhai and Lua (others?)
- WGPU integration - to be used in conjunction with scripting. 
- Arc'd String Keys and other key types
- Add error conditions and messages for conflicting feature specifications.
- Vector comparison functionality
- Vector search functionality
- Add more namespace implementations using collection types like [Moka](https://crates.io/crates/moka) and std::Hashmap etc.
- Complete namespace implementations for the store and atomicity levels.
- Add regex storage
- Add the ability to search for namespace items using a regex that is either stored or part of the request i.e. Mass deletion based on whether on not a given regex matches etc.
- Add the ability to remove namespace items based on weather or not it starts with or ends with a certain sequence of values or otherwise contains this sequence of values.
- Clean up the code.
- Document the code. 

## Possible:

- Option Types (Unlikely, unless it's Vec\<Option\<T\>\>)

<br/>

## Known Issues And Types To Be Removed Or Disabled

| Feature                   | Issue       |
| -----------               | ----------- |
| i64                       | Broken - incompatible with GraphQL/async-graphql |
| i128                      | Broken - incompatible with GraphQL/async-graphql |
| isize                     | To be removed |
| u64                       | Broken - incompatible with GraphQL/async-graphql |
| u128                      | Broken - incompatible with GraphQL/async-graphql |
| usize                     | To be removed |
| SelectedType              | A GraphQL union of whatever else was selected except Whatever - To be removed/disabled |
| Vec_i64                   | Broken - incompatible with GraphQL/async-graphql |
| Vec_i128                  | Broken - incompatible with GraphQL/async-graphql |
| Vec_isize                 | To be removed |
| VecSelectedType           | To be removed/disabled |
| Vec_u64                   | Broken - incompatible with GraphQL/async-graphql |
| Vec_u128                  | Broken - incompatible with GraphQL/async-graphql |
| Vec_usize                 | To be removed           |

So GraphQL doesn't support intergers with sizes [above 32 bits](https://spec.graphql.org/October2021/#sec-Int).

I'll implement a work around for this issue.

The isize and usize types are redundant.

SelectedType was implemented for an experiment in asynchronous queries.

I think I got to the trial implementation stage with this feature and I may revisit it so I'll remove SelectedType and VecSelectedType in the next version but keep the code in the repository.

<br/>

## About Generic Permutations

In order to control the amount of work requred certain classes of type will and won't be implemented:

- No option unless in other gernerics e.g. Vec\<Option\<T\>\>. While having a base level Option could be a convenient way to reserve a key in a namespace, for the time being at least, base level Option namespaces wont be added.
- With hashmaps and other generic types that take multiple augments you can have all manner of different combinations. So the keys of namespace implementations will probably be constrained to what you can select with the available features and not arbitrary values (sub-namespaces).

<br/>

This project has come along way since its inception in mid-2022 as a Redis-like cache and it has a long way to go still. Please use with the understanding that this project is a work in progress.

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

