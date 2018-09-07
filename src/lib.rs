// Source adopted from
// https://github.com/tildeio/helix-website/blob/master/crates/word_count/src/lib.rs
#![feature(specialization)]

#[macro_use]
extern crate pyo3;

extern crate gp_daq;

use std::fs::File;
use pyo3::prelude::*;
use pyo3::IntoPyDict;


use gp_daq::io::event_file::EventFile as RawEventFile;
use gp_daq::io::event_file::EventHeader as RawEventHeader;
use gp_daq::io::event_file::FileHeader as RawFileHeader;

#[pyclass]
/// Represents a file that can be searched
struct EventFile {
    content:RawEventFile,
    token:PyToken,
}

#[pymethods]
impl EventFile {
    #[new]
    fn __new__(obj: &PyRawObject, path: String) -> PyResult<()> {
        let mut f=File::open(path)?;
        let ef=RawEventFile::read_from(&mut f).unwrap();
        obj.init(|token| EventFile {
            content:ef,
            token:token,
        });
        Ok(())
    }

    #[getter]
    pub fn header(&self)->PyResult<FileHeader>{
        Ok(FileHeader{content:self.content.header.clone()})
    }

}

#[pyclass]
struct FileHeader{
    content:RawFileHeader,
}

#[pymethods]
impl FileHeader{
}

impl IntoPyDict for FileHeader{
    fn into_py_dict(self, py:Python)->&PyDict{
        let pyd=PyDict::new(py);
        pyd.set_item("length", self.content.basic_header.length).unwrap();
        pyd.set_item("runnr", self.content.basic_header.runnr).unwrap();
        pyd.set_item("run_mod", self.content.basic_header.run_mod).unwrap();
        pyd.set_item("serial", self.content.basic_header.seral).unwrap();
        pyd.set_item("first_event", self.content.basic_header.first_event).unwrap();
        pyd.set_item("first_event_sec", self.content.basic_header.first_event_sec).unwrap();
        pyd.set_item("last_event", self.content.basic_header.last_event).unwrap();
        pyd.set_item("last_event_sec", self.content.basic_header.last_event_sec).unwrap();
        pyd
    }
}


#[pymodinit]
fn pyef(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<EventFile>()?;
    m.add_class::<FileHeader>()?;
    Ok(())
}
