## 0.2.2 (Unreleased)

- Added `StackedAttrArrayW` data types.
- Added `ret`, `ref`, and `com` as type casting functions. Recommended use is only inside the classbody, as they cause overhead in `on_xxx` functions
- Changed `Manager` to only allow using `RetrieverRef`s
- Added `DebugByteStream` which logs the bytes it reads for each specified retriever to `bfp.log`
- Changed `DefaultValueError` to `DefaultAttributeError`, used when trying to instantiate a `BaseStruct` from default without having default values
- Added a `_dbg_repr` to base struct which can print partially initialised `BaseStruct` objects

## 0.2.1

- `RetrieverRef` can now accept an arbitrary number of retrievers to be able to reference data from sub structs.
- `RetrieverRef` can now accept an `int` as part of its arguments to index lists where required
- `RetrieverRef` needs to be given at least one argument
- `RetrieverRef` can now accept `Retriever` or `Combiner` or other `Ref`s as arguments
- `RetrieverCombiner` must be given at least one argument
- `RetrieverCombiner` can now accept `Retriever` or `Ref`s or other `Combiner`s as arguments
- `VersionError` now also subclasses `AttributeError`
- `DefaultValueError` now also subclasses `ValueError`
- Diffing structs now returns a dict of name-value pairs instead of just a list of retrievers.
  - Note: This is an expensive operation and should only be used for debug purposes. diff's behaviour is subject to change!
- Added `StrArray8s`, `StrArray16s`, `StrArray32s` and `StrArray64s` datatypes
- Added `Manager` superclass for creating grouped retriever references and functions on them. Use this to provide a more coherent API
    for struct modification when the internal struct is messy
- Added docstrings to datatypes, explaining their expected layouts

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
