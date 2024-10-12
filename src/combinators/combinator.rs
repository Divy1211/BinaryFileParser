use pyo3::PyResult;

use crate::retrievers::retriever::Retriever;
use crate::types::parseable_type::ParseableType;
use crate::types::version::Version;

pub trait Combinator {
     fn run(
         &self,
         retrievers: &Vec<Retriever>,
         data: &mut Vec<Option<ParseableType>>,
         repeats: &mut Vec<Option<isize>>,
         ver: &Version
     ) -> PyResult<()>;
 }