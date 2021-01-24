function addRow(table_id) {
  let table = document.getElementById(table_id);
  let new_row = table.insertRow(-1);
  let cell_taskname = new_row.insertCell(0);
  let cell_time = new_row.insertCell(1);
  let current_row_number = table.rows.length;

  cell_taskname.innerHTML = `
	<input type="text" id="task${current_row_number}" name="task${current_row_number}" list="tasks" autocomplete="off" placeholder="Task" required \>`;
  cell_time.innerHTML = `
            <input
              type="number"
              id="time_hours${current_row_number}"
			  class="time_hours"
              name="time_hours${current_row_number}"
              min="0"
              max="24"
			  size="2"
              placeholder="hours"
              required
            /> :
            <input
              type="number"
              id="time_minutes${current_row_number}"
			  class="time_minutes"
              name="time_minutes${current_row_number}"
              min="0"
              max="59"
			  size="2"
              placeholder="min"
              required
            />`;
}

function deleteRow(table_id) {
  let table = document.getElementById(table_id);
  if (table.rows.length > 1) {
    table.deleteRow(-1);
  }
}

function clear_task_table() {
  let table = document.getElementById("task_tbl");
  let number_rows = table.rows.length;
  for (let i = 0; i < number_rows - 1; i++) {
    table.deleteRow(-1);
  }
}

function minutes_to_hour_string(minutes) {
  let minimum_time_hours = Math.floor(minutes / 60)
    .toString()
    .padStart(2, "0");
  let minimum_time_minutes = (minutes % 60).toString().padStart(2, "0");
  return `${minimum_time_hours}:${minimum_time_minutes}`;
}

function delete_task(task_id) {
  let request = new XMLHttpRequest();
  request.onload = function () {
    set_task_table();
  };
  request.open("POST", "php/ajax_delete_task.php", true);
  request.setRequestHeader("Content-type", "application/x-www-form-urlencoded");
  request.send(`task_id=${task_id}`);
  
}

function set_task_table() {
  let request = new XMLHttpRequest();
  request.onload = function () {
    clear_task_table();
    response = JSON.parse(this.responseText);
    let table = document.getElementById("task_tbl");
    for (let i = 0; i < response.length; i++) {
      let new_row = table.insertRow(-1);
      let cell_name = new_row.insertCell(0);
      let cell_minimum_time = new_row.insertCell(1);
      let cell_time_spent = new_row.insertCell(2);
      let cell_finished = new_row.insertCell(3);
      let cell_delete = new_row.insertCell(4);
      cell_name.innerHTML = response[i].name;
      cell_minimum_time.innerHTML = minutes_to_hour_string(
        response[i].minimum_time
      );
      cell_time_spent.innerHTML = minutes_to_hour_string(
        response[i].time_spent
      );
      cell_finished.innerHTML = response[i].finished == 0 ? "✗" : "✓";
      let tasks_id = response[i].id
      cell_delete.innerHTML = `<button type="button" onclick="delete_task(${tasks_id})">Delete</button>`;
    }
  };
  request.open("POST", "php/ajax_get_tasks.php", true);
  request.setRequestHeader("Content-type", "application/x-www-form-urlencoded");
  let date_field = document.getElementById("date");
  let date = date_field.value;
  request.send(`date=${date}`);
}
