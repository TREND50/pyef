#![allow(clippy::cast_ptr_alignment)]
#![feature(custom_attribute)]
// Source adopted from
// https://github.com/tildeio/helix-website/blob/master/crates/word_count/src/lib.rs
#![feature(specialization)]

extern crate pyo3;

extern crate gp_daq;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

use std::fs::File;

use gp_daq::io::event_file::EventFile as RawEventFile;
use gp_daq::io::event_file::FileHeader as RawFileHeader;

use gp_daq::io::event_file::Event as RawEvent;
use gp_daq::io::event_file::EventHeader as RawEventHeader;

use gp_daq::io::event_file::LocalStation as RawLocalStation;
use gp_daq::io::event_file::LocalStationHeader as RawLocalStationHeader;

#[pyclass]
/// Represents a file that can be searched
pub struct EventFile {
    content: RawEventFile,
}

#[pymethods]
impl EventFile {
    #[getter]
    fn header(&self) -> PyResult<FileHeader> {
        Ok(FileHeader {
            content: self.content.header.clone(),
        })
    }

    #[getter]
    fn event_list(&self) -> PyResult<Vec<Event>> {
        let v = self
            .content
            .event_list
            .iter()
            .map(|e| Event { content: e.clone() })
            .collect();
        Ok(v)
    }

    pub fn __str__(&self) -> PyResult<String> {
        Ok(format!("{}", self.content))
    }
    pub fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{}", self.content))
    }

    pub fn display(&self) -> PyResult<()> {
        println!("{}", self.content);
        Ok(())
    }
}

#[pyclass]
struct FileHeader {
    content: RawFileHeader,
}

#[pymethods]
impl FileHeader {
    #[getter]
    pub fn length(&self) -> PyResult<u32> {
        Ok(self.content.basic_header.length)
    }
    #[getter]
    pub fn runnr(&self) -> PyResult<u32> {
        Ok(self.content.basic_header.runnr)
    }
    #[getter]
    pub fn run_mod(&self) -> PyResult<u32> {
        Ok(self.content.basic_header.run_mod)
    }
    #[getter]
    pub fn serial(&self) -> PyResult<u32> {
        Ok(self.content.basic_header.seral)
    }
    #[getter]
    pub fn first_event(&self) -> PyResult<u32> {
        Ok(self.content.basic_header.first_event)
    }
    #[getter]
    pub fn first_event_sec(&self) -> PyResult<u32> {
        Ok(self.content.basic_header.first_event_sec)
    }
    #[getter]
    pub fn last_event(&self) -> PyResult<u32> {
        Ok(self.content.basic_header.last_event)
    }
    #[getter]
    pub fn last_event_sec(&self) -> PyResult<u32> {
        Ok(self.content.basic_header.last_event_sec)
    }
    #[getter]
    pub fn additional_header(&self) -> PyResult<Vec<u32>> {
        Ok(self.content.additional_header.clone())
    }

    pub fn __str__(&self) -> PyResult<String> {
        Ok(format!("{}", self.content))
    }
    pub fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{}", self.content))
    }

    pub fn display(&self) -> PyResult<()> {
        println!("{}", self.content);
        Ok(())
    }
}

#[pyclass]
struct Event {
    content: RawEvent,
}

#[pymethods]
impl Event {
    #[getter]
    fn header(&self) -> PyResult<EventHeader> {
        Ok(EventHeader {
            content: self.content.header,
        })
    }

    #[getter]
    fn local_station_list(&self) -> PyResult<Vec<LocalStation>> {
        Ok(self
            .content
            .local_station_list
            .iter()
            .map(|e| LocalStation { content: e.clone() })
            .collect())
    }

    pub fn __str__(&self) -> PyResult<String> {
        Ok(format!("{}", self.content))
    }
    pub fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{}", self.content))
    }

    pub fn display(&self) -> PyResult<()> {
        println!("{}", self.content);
        Ok(())
    }
}

#[pyclass]
struct EventHeader {
    content: RawEventHeader,
}

#[pymethods]
impl EventHeader {
    #[getter]
    pub fn header_length(&self) -> PyResult<u32> {
        Ok(self.content.header_length)
    }
    #[getter]
    pub fn runnr(&self) -> PyResult<u32> {
        Ok(self.content.runnr)
    }
    #[getter]
    pub fn eventnr(&self) -> PyResult<u32> {
        Ok(self.content.eventnr)
    }
    #[getter]
    pub fn t3eventnr(&self) -> PyResult<u32> {
        Ok(self.content.t3eventnr)
    }
    #[getter]
    pub fn first_ls(&self) -> PyResult<u32> {
        Ok(self.content.first_ls)
    }
    #[getter]
    pub fn event_sec(&self) -> PyResult<u32> {
        Ok(self.content.event_sec)
    }
    #[getter]
    pub fn event_nsec(&self) -> PyResult<u32> {
        Ok(self.content.event_nsec)
    }
    #[getter]
    pub fn event_type(&self) -> PyResult<u16> {
        Ok(self.content.event_type)
    }
    #[getter]
    pub fn event_vers(&self) -> PyResult<u16> {
        Ok(self.content.event_vers)
    }
    #[getter]
    pub fn ad1(&self) -> PyResult<u32> {
        Ok(self.content.ad1)
    }
    #[getter]
    pub fn ad2(&self) -> PyResult<u32> {
        Ok(self.content.ad2)
    }
    #[getter]
    pub fn ls_cnt(&self) -> PyResult<u32> {
        Ok(self.content.ls_cnt)
    }

    pub fn __str__(&self) -> PyResult<String> {
        Ok(format!("{}", self.content))
    }
    pub fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{}", self.content))
    }

    pub fn display(&self) -> PyResult<()> {
        println!("{}", self.content);
        Ok(())
    }
}

#[pyclass]
struct LocalStationHeader {
    content: RawLocalStationHeader,
}

#[pymethods]
impl LocalStationHeader {
    #[getter]
    pub fn length(&self) -> PyResult<u16> {
        Ok(self.content.length)
    }
    #[getter]
    pub fn event_nr(&self) -> PyResult<u16> {
        Ok(self.content.event_nr)
    }
    #[getter]
    pub fn ls_id(&self) -> PyResult<u16> {
        Ok(self.content.ls_id)
    }
    #[getter]
    pub fn header_length(&self) -> PyResult<u16> {
        Ok(self.content.header_length)
    }
    #[getter]
    pub fn gps_seconds(&self) -> PyResult<u32> {
        Ok(self.content.gps_seconds)
    }
    #[getter]
    pub fn gps_nanoseconds(&self) -> PyResult<u32> {
        Ok(self.content.gps_nanoseconds)
    }
    #[getter]
    pub fn trigger_flag(&self) -> PyResult<u16> {
        Ok(self.content.trigger_flag)
    }
    #[getter]
    pub fn trigger_pos(&self) -> PyResult<u16> {
        Ok(self.content.trigger_pos)
    }
    #[getter]
    pub fn sampling_freq(&self) -> PyResult<u16> {
        Ok(self.content.sampling_freq)
    }
    #[getter]
    pub fn channel_mask(&self) -> PyResult<u16> {
        Ok(self.content.channel_mask)
    }
    #[getter]
    pub fn adc_resolution(&self) -> PyResult<u16> {
        Ok(self.content.adc_resolution)
    }
    #[getter]
    pub fn trace_length(&self) -> PyResult<u16> {
        Ok(self.content.trace_length)
    }
    #[getter]
    pub fn version(&self) -> PyResult<u16> {
        Ok(self.content.version)
    }

    pub fn __str__(&self) -> PyResult<String> {
        Ok(format!("{}", self.content))
    }
    pub fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{}", self.content))
    }

    pub fn display(&self) -> PyResult<()> {
        println!("{}", self.content);
        Ok(())
    }
}

#[pyclass]
struct LocalStation {
    content: RawLocalStation,
}

#[pymethods]
impl LocalStation {
    #[getter]
    pub fn header(&self) -> PyResult<LocalStationHeader> {
        Ok(LocalStationHeader {
            content: self.content.header,
        })
    }
    #[getter]
    pub fn header_data(&self) -> PyResult<Vec<u16>> {
        Ok(self.content.header_data.clone())
    }
    #[getter]
    pub fn adc_buffer(&self) -> PyResult<Vec<u16>> {
        Ok(self.content.adc_buffer.clone())
    }

    pub fn __str__(&self) -> PyResult<String> {
        Ok(format!("{}", self.content))
    }
    pub fn __repr__(&self) -> PyResult<String> {
        Ok(format!("{}", self.content))
    }

    pub fn display(&self) -> PyResult<()> {
        println!("{}", self.content);
        Ok(())
    }
}

#[pyfunction]
pub fn read_file(name: &str) -> PyResult<EventFile> {
    let mut f = File::open(name).unwrap();
    Ok(EventFile {
        content: RawEventFile::read_from(&mut f).unwrap(),
    })
}

#[pymodule]
fn native(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<EventFile>()?;
    m.add_class::<FileHeader>()?;
    m.add_class::<Event>()?;
    m.add_class::<EventHeader>()?;
    m.add_class::<LocalStation>()?;
    m.add_class::<LocalStationHeader>()?;
    m.add_wrapped(wrap_pyfunction!(read_file))?;
    Ok(())
}
