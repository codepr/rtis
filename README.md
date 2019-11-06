RTIS
====

**R**ust **T**ext-**I**ndexer **S**erver, first attempt to build something in Rust, trivial
implementation of a text-indexer server, return results in simple JSON format.

Single thread HTTP server, exposes two methods:

- `POST` to add new text to be indexed
- `GET` to query the server for results

No dependencies, being that simple of a project, it's all built from scratch.

### Improvements

- `async/.await` to accept multiple connections
- improve the indexing algorithm
