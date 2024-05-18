## 0.2.1

- `RetrieverRef` can now accept an arbitrary number of retrievers to be able to reference data from sub structs.
- `RetrieverRef` can now accept an `int` as part of its arguments to index lists where required
- `RetrieverRef` needs to be given at least one argument
- `RetrieverCombiner` must be given at least one argument

## 0.2.0

- For performance reasons, a `parent` reference is no longer available. To synchronise duplicate data in different parts of a file, the data source higher in the struct hierarchy is to be made responsible for performing the synchronisation.
- `RefList` has been removed since parent references are no longer available.
- `struct_ver` is now a read-only property and the functionality for changing a struct's version is no longer available and will be re-added in a future update.
- A `Retriever`'s repeat value is interpreted as follows:
  - repeat = 1 (default) means its data type is read once.
  - repeat = 0 or > 1 means it is a list of its data type.
  - repeat = -1 means the value does not exist and is skipped. (set to `None`)
  - repeat set via `set_repeat` will read a list of its data type, even for repeat = 1.
    - Note: If a single value is only conditionally present, then its repeat should default to 1, and should be set to -1 dynamically when it is intended to be absent for the desired behaviour. Setting a repeat to 1 dynamically will make it a singleton list!
