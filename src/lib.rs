use pyo3::prelude::*;

#[pyfunction]
fn get_amount(
	claimed_amounts: Vec<i32>,
	max_enlargement: i32,
	amount: i32,
	min_last_amount_difference: i32,
) -> PyResult<Option<i32>> {
	if !claimed_amounts.contains(&amount) {
		return Ok(Some(amount));
	}

	let last_claimed_amount = *claimed_amounts.last().unwrap();

	Ok((amount..amount + max_enlargement).find(|&new_amount| {
		!claimed_amounts.contains(&new_amount)
			&& (new_amount < last_claimed_amount - min_last_amount_difference || new_amount > last_claimed_amount + min_last_amount_difference)
	}))
}

#[pymodule]
fn payments_api_extensions(_py: Python, m: &PyModule) -> PyResult<()> {
	m.add_function(wrap_pyfunction!(get_amount, m)?)?;
	Ok(())
}
