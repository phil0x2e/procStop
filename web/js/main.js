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

function set_task_table() {
  let request = new XMLHttpRequest();
  request.onload = function () {
    response = JSON.parse(this.responseText);
    let table = document.getElementById("task_tbl");
    for (let i = 0; i < response.length; i++) {
      let new_row = table.insertRow(-1);
      let cell_name = new_row.insertCell(0);
      let cell_time = new_row.insertCell(1);
      let cell_time_spent = new_row.insertCell(2);
      let cell_finished = new_row.insertCell(3);
      cell_name.innerHTML = response[i].name;
      cell_time.innerHTML = response[i].time_to_spend;
      cell_time_spent.innerHTML = response[i].time_already_spent;
      cell_finished.innerHTML = response[i].finished == 0 ? "✗" : "✓";
    }
  };
  request.open("POST", "../php/ajax_get_tasks.php", true);
  request.setRequestHeader("Content-type", "application/x-www-form-urlencoded");
  let date_field = document.getElementById("date");
  let date = date_field.value;
  request.send(`day=${date}`);
}
