mod algos;
mod collections;
use collections::timeseries::TimeSeries;

use pyo3::prelude::*;
use pyo3::prelude::PyModule;
use pyo3::{wrap_pyfunction, PyResult, Python};
use pyo3::types::PyDict;

use crate::algos::stl::{stl, time_series_outliers};

#[pyfunction]
pub fn timeseries(data: Vec<Vec<f32>>) -> TimeSeries {
    TimeSeries::new(data)
}

#[pyfunction]
pub fn decompose(data: TimeSeries, py_kwargs: Option<&PyDict>) -> (TimeSeries, TimeSeries, TimeSeries) {
    if py_kwargs.is_some() {
        match py_kwargs.unwrap().get_item("period") {
            Some(item) => stl(data, &item.extract::<usize>().unwrap()),
            None => stl(data, &Option::<usize>::None)
        }
    }

    else {
        stl(data, &Option::<usize>::None)
    }
}

#[pyfunction]
pub fn timeseries_outliers(data: TimeSeries) -> Vec<(usize, f32)> {
    time_series_outliers(data)
}

#[pymodule]
fn oddity(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<TimeSeries>()?;
    m.add_function(wrap_pyfunction!(timeseries, m)?)?;
    m.add_function(wrap_pyfunction!(decompose, m)?)?;
    m.add_function(wrap_pyfunction!(timeseries_outliers, m)?)?;

    Ok(())
}
