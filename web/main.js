function addRow(table_id) {
  let table = document.getElementById(table_id);
  let row = table.insertRow(-1);
  let cell1 = row.insertCell(0);
  let cell2 = row.insertCell(1);
  let current_row = table.rows.length - 1;

  cell1.innerHTML = `
	<input type="text" id="task${current_row}" name="task${current_row}" list="tasks">
    <datalist id="tasks">
      <option value="Arbeit">
      <option value="Uni">
	</datalist>
	`;
  cell2.innerHTML = `
	<input type="number" id="time${current_row}" name="time${current_row}" min="0" max="1440">min
	`;
}

function deleteRow(table_id) {
  let table = document.getElementById(table_id);
  if (table.rows.length > 2) {
    table.deleteRow(-1);
  }
}
