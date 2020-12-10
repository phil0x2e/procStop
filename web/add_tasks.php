<!DOCTYPE html>
<html>
  <head>
    <title>ProcStop - Add Tasks</title>
    <meta charset="UTF-8" />
    <script src="js/main.js"></script>
	<link rel="stylesheet" href="css/main.css"/>
  </head>
  <body>
    <ul class="navbar">
      <li><a href="index.html">Home</a></li>
      <li><a class="active" href="add_tasks.php">Add Tasks</a></li>
      <li><a href="view_tasks.php">View Tasks</a></li>
    </ul>
    <h1>Add new Tasks</h1>
    <form action="php/add_tasks.php" method="post">
      <label for="date"><b>Date:</b></label>
      <input type="date" id="date" name="date" value="<?php echo date("Y-m-d");?>"/><br /><br />
      <table id="tbl-inputs">
        <tr>
          <th><label for="task1">Task</label></th>
          <th><label for="time1">Time</label></th>
        </tr>
        <tr>
          <td>
            <input type="text" name="task1" id="task1" list="tasks" autocomplete="off"/>
            <datalist id="tasks">
            <?php
            function get_task_names($db) {
                $sql = "SELECT name FROM tasknames;";

                if (!($stmt = $db->prepare($sql))) {
                    return array();
                }

                if (!$stmt->execute()) {
                    return array();
                }
                $rows = $stmt->fetchAll(PDO::FETCH_ASSOC);
                return $rows;
            }
            $db = new PDO("sqlite:database.sqlite3");
            foreach (get_task_names($db) as $row) {
                echo "<option value='" . $row["name"] . "'></option>";
            }
            ?>
            </datalist>
          </td>
          <td>
            <input
              type="number"
              id="time1"
              name="time1"
              min="0"
              max="1440"
            />min
          </td>
        </tr>
      </table>
      <button type="button" onclick="addRow('tbl-inputs');">➕</button>
      <button type="button" onclick="deleteRow('tbl-inputs');">➖</button>
      <button type="submit">Submit</button>
    </form>
  </body>
</html>


