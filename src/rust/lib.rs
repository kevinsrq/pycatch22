use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

// ---------------------------------------------------------------------
// -------------------- Helper wrapper functions -----------------------
// ---------------------------------------------------------------------

fn convert_py_list_to_vec(data: &Bound<'_, PyAny>) -> PyResult<Vec<f64>> {
    if let Ok(list) = data.downcast::<pyo3::types::PyList>() {
        list.iter()
            .map(|item| item.extract::<f64>())
            .collect::<PyResult<Vec<f64>>>()
    } else if let Ok(tuple) = data.downcast::<pyo3::types::PyTuple>() {
        tuple.iter()
            .map(|item| item.extract::<f64>())
            .collect::<PyResult<Vec<f64>>>()
    } else {
        Err(PyValueError::new_err("Expected list or tuple"))
    }
}

// ---------------------------------------------------------------------
// ----------------------- Python wrapper functions --------------------
// ---------------------------------------------------------------------

#[pyfunction]
fn DN_Mean(data: &Bound<'_, PyAny>) -> PyResult<f64> {
    let vec_data = convert_py_list_to_vec(data)?;
    Ok(catch22::compute(&vec_data, 22))
}

#[pyfunction]
fn DN_Spread_Std(data: &Bound<'_, PyAny>) -> PyResult<f64> {
    let vec_data = convert_py_list_to_vec(data)?;
    Ok(catch22::compute(&vec_data, 23))
}

#[pyfunction]
fn DN_HistogramMode_5(data: &Bound<'_, PyAny>) -> PyResult<f64> {
    let vec_data = convert_py_list_to_vec(data)?;
    let normalized = catch22::zscore(&vec_data);
    Ok(catch22::compute(&normalized, 2))
}

#[pyfunction]
fn DN_HistogramMode_10(data: &Bound<'_, PyAny>) -> PyResult<f64> {
    let vec_data = convert_py_list_to_vec(data)?;
    let normalized = catch22::zscore(&vec_data);
    Ok(catch22::compute(&normalized, 3))
}

#[pyfunction]
fn CO_f1ecac(data: &Bound<'_, PyAny>) -> PyResult<f64> {
    let vec_data = convert_py_list_to_vec(data)?;
    let normalized = catch22::zscore(&vec_data);
    Ok(catch22::compute(&normalized, 5))
}

#[pyfunction]
fn CO_FirstMin_ac(data: &Bound<'_, PyAny>) -> PyResult<i64> {
    let vec_data = convert_py_list_to_vec(data)?;
    let normalized = catch22::zscore(&vec_data);
    Ok(catch22::compute(&normalized, 6) as i64)
}

#[pyfunction]
fn CO_HistogramAMI_even_2_5(data: &Bound<'_, PyAny>) -> PyResult<f64> {
    let vec_data = convert_py_list_to_vec(data)?;
    let normalized = catch22::zscore(&vec_data);
    Ok(catch22::compute(&normalized, 7))
}

#[pyfunction]
fn CO_trev_1_num(data: &Bound<'_, PyAny>) -> PyResult<f64> {
    let vec_data = convert_py_list_to_vec(data)?;
    let normalized = catch22::zscore(&vec_data);
    Ok(catch22::compute(&normalized, 8))
}

#[pyfunction]
fn MD_hrv_classic_pnn40(data: &Bound<'_, PyAny>) -> PyResult<f64> {
    let vec_data = convert_py_list_to_vec(data)?;
    let normalized = catch22::zscore(&vec_data);
    Ok(catch22::compute(&normalized, 12))
}

#[pyfunction]
fn SB_BinaryStats_mean_longstretch1(data: &Bound<'_, PyAny>) -> PyResult<f64> {
    let vec_data = convert_py_list_to_vec(data)?;
    let normalized = catch22::zscore(&vec_data);
    Ok(catch22::compute(&normalized, 14))
}

#[pyfunction]
fn SB_TransitionMatrix_3ac_sumdiagcov(data: &Bound<'_, PyAny>) -> PyResult<f64> {
    let vec_data = convert_py_list_to_vec(data)?;
    let normalized = catch22::zscore(&vec_data);
    Ok(catch22::compute(&normalized, 20))
}

#[pyfunction]
fn PD_PeriodicityWang_th0_01(data: &Bound<'_, PyAny>) -> PyResult<i64> {
    let vec_data = convert_py_list_to_vec(data)?;
    let normalized = catch22::zscore(&vec_data);
    Ok(catch22::compute(&normalized, 21) as i64)
}

#[pyfunction]
fn CO_Embed2_Dist_tau_d_expfit_meandiff(data: &Bound<'_, PyAny>) -> PyResult<f64> {
    let vec_data = convert_py_list_to_vec(data)?;
    let normalized = catch22::zscore(&vec_data);
    Ok(catch22::compute(&normalized, 4))
}

#[pyfunction]
fn IN_AutoMutualInfoStats_40_gaussian_fmmi(data: &Bound<'_, PyAny>) -> PyResult<f64> {
    let vec_data = convert_py_list_to_vec(data)?;
    let normalized = catch22::zscore(&vec_data);
    Ok(catch22::compute(&normalized, 11))
}

#[pyfunction]
fn FC_LocalSimple_mean1_tauresrat(data: &Bound<'_, PyAny>) -> PyResult<f64> {
    let vec_data = convert_py_list_to_vec(data)?;
    let normalized = catch22::zscore(&vec_data);
    Ok(catch22::compute(&normalized, 9))
}

#[pyfunction]
fn DN_OutlierInclude_p_001_mdrmd(data: &Bound<'_, PyAny>) -> PyResult<f64> {
    let vec_data = convert_py_list_to_vec(data)?;
    let normalized = catch22::zscore(&vec_data);
    Ok(catch22::compute(&normalized, 1))
}

#[pyfunction]
fn DN_OutlierInclude_n_001_mdrmd(data: &Bound<'_, PyAny>) -> PyResult<f64> {
    let vec_data = convert_py_list_to_vec(data)?;
    let normalized = catch22::zscore(&vec_data);
    Ok(catch22::compute(&normalized, 0))
}

#[pyfunction]
fn SP_Summaries_welch_rect_area_5_1(data: &Bound<'_, PyAny>) -> PyResult<f64> {
    let vec_data = convert_py_list_to_vec(data)?;
    let normalized = catch22::zscore(&vec_data);
    Ok(catch22::compute(&normalized, 18))
}

#[pyfunction]
fn SB_BinaryStats_diff_longstretch0(data: &Bound<'_, PyAny>) -> PyResult<f64> {
    let vec_data = convert_py_list_to_vec(data)?;
    let normalized = catch22::zscore(&vec_data);
    Ok(catch22::compute(&normalized, 13))
}

#[pyfunction]
fn SB_MotifThree_quantile_hh(data: &Bound<'_, PyAny>) -> PyResult<f64> {
    let vec_data = convert_py_list_to_vec(data)?;
    let normalized = catch22::zscore(&vec_data);
    Ok(catch22::compute(&normalized, 15))
}

#[pyfunction]
fn SC_FluctAnal_2_rsrangefit_50_1_logi_prop_r1(data: &Bound<'_, PyAny>) -> PyResult<f64> {
    let vec_data = convert_py_list_to_vec(data)?;
    let normalized = catch22::zscore(&vec_data);
    Ok(catch22::compute(&normalized, 16))
}

#[pyfunction]
fn SC_FluctAnal_2_dfa_50_1_2_logi_prop_r1(data: &Bound<'_, PyAny>) -> PyResult<f64> {
    let vec_data = convert_py_list_to_vec(data)?;
    let normalized = catch22::zscore(&vec_data);
    Ok(catch22::compute(&normalized, 17))
}

#[pyfunction]
fn SP_Summaries_welch_rect_centroid(data: &Bound<'_, PyAny>) -> PyResult<f64> {
    let vec_data = convert_py_list_to_vec(data)?;
    let normalized = catch22::zscore(&vec_data);
    Ok(catch22::compute(&normalized, 19))
}

#[pyfunction]
fn FC_LocalSimple_mean3_stderr(data: &Bound<'_, PyAny>) -> PyResult<f64> {
    let vec_data = convert_py_list_to_vec(data)?;
    let normalized = catch22::zscore(&vec_data);
    Ok(catch22::compute(&normalized, 10))
}

// ---------------------------------------------------------------------
// ------------------------ Module definition --------------------------
// ---------------------------------------------------------------------

#[pymodule]
fn catch22_C(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(DN_Mean, m)?)?;
    m.add_function(wrap_pyfunction!(DN_Spread_Std, m)?)?;
    m.add_function(wrap_pyfunction!(DN_HistogramMode_5, m)?)?;
    m.add_function(wrap_pyfunction!(DN_HistogramMode_10, m)?)?;
    m.add_function(wrap_pyfunction!(CO_f1ecac, m)?)?;
    m.add_function(wrap_pyfunction!(CO_FirstMin_ac, m)?)?;
    m.add_function(wrap_pyfunction!(CO_HistogramAMI_even_2_5, m)?)?;
    m.add_function(wrap_pyfunction!(CO_trev_1_num, m)?)?;
    m.add_function(wrap_pyfunction!(MD_hrv_classic_pnn40, m)?)?;
    m.add_function(wrap_pyfunction!(SB_BinaryStats_mean_longstretch1, m)?)?;
    m.add_function(wrap_pyfunction!(SB_TransitionMatrix_3ac_sumdiagcov, m)?)?;
    m.add_function(wrap_pyfunction!(PD_PeriodicityWang_th0_01, m)?)?;
    m.add_function(wrap_pyfunction!(CO_Embed2_Dist_tau_d_expfit_meandiff, m)?)?;
    m.add_function(wrap_pyfunction!(IN_AutoMutualInfoStats_40_gaussian_fmmi, m)?)?;
    m.add_function(wrap_pyfunction!(FC_LocalSimple_mean1_tauresrat, m)?)?;
    m.add_function(wrap_pyfunction!(DN_OutlierInclude_p_001_mdrmd, m)?)?;
    m.add_function(wrap_pyfunction!(DN_OutlierInclude_n_001_mdrmd, m)?)?;
    m.add_function(wrap_pyfunction!(SP_Summaries_welch_rect_area_5_1, m)?)?;
    m.add_function(wrap_pyfunction!(SB_BinaryStats_diff_longstretch0, m)?)?;
    m.add_function(wrap_pyfunction!(SB_MotifThree_quantile_hh, m)?)?;
    m.add_function(wrap_pyfunction!(SC_FluctAnal_2_rsrangefit_50_1_logi_prop_r1, m)?)?;
    m.add_function(wrap_pyfunction!(SC_FluctAnal_2_dfa_50_1_2_logi_prop_r1, m)?)?;
    m.add_function(wrap_pyfunction!(SP_Summaries_welch_rect_centroid, m)?)?;
    m.add_function(wrap_pyfunction!(FC_LocalSimple_mean3_stderr, m)?)?;
    Ok(())
}
