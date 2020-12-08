function addRow(table_id) {
  let table = document.getElementById(table_id);
  let new_row = table.insertRow(-1);
  let cell1 = new_row.insertCell(0);
  let cell2 = new_row.insertCell(1);
  let current_row_number = table.rows.length - 1;

  cell1.innerHTML = `
	<input type="text" id="task${current_row_number}" name="task${current_row_number}" list="tasks" autocomplete="off">`;
  cell2.innerHTML = `
	<input type="number" id="time${current_row_number}" name="time${current_row_number}" min="0" max="1440">min
	`;
}

function deleteRow(table_id) {
  let table = document.getElementById(table_id);
  if (table.rows.length > 2) {
    table.deleteRow(-1);
  }
}
