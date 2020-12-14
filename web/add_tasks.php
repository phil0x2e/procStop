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
    <form action="php/add_tasks_backend.php" method="post">
      <label for="date"><b>Date:</b></label>
      <input type="date" id="date" name="date" value="<?php echo date(
          "Y-m-d",
      ); ?>" required/><br/><br />
      <table id="tbl-inputs">
        <tr>
          <td>
            <input type="text" name="task1" id="task1" list="tasks" placeholder="Task" autocomplete="off" autofocus required/>
            <datalist id="tasks">
            <?php
            function get_task_names($db) {
                $sql = "SELECT name FROM tasknames;";

                if (!($stmt = $db->prepare($sql))) {
                    return [];
                }

                if (!$stmt->execute()) {
                    return [];
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
              id="time_hours1"
              name="time_hours1"
			  class="time_hours"
              min="0"
              max="24"
			  size="2"
              placeholder="hours"
              required
            /> :
            <input
              type="number"
              id="time_minutes1"
              name="time_minutes1"
			  class="time_minutes"
              min="0"
              max="59"
			  size="2"
              placeholder="min"
              required
            />
          </td>
        </tr>
      </table>
      <button type="button" onclick="addRow('tbl-inputs');">➕</button>
      <button type="button" onclick="deleteRow('tbl-inputs');">➖</button>
      <button type="submit">Submit</button>
    </form><br><br>
    <?php if (isset($_GET["success"]) && $_GET["success"] == "true") {
        echo "<span align='center' class='success'>Successfully inserted Tasks!</span>";
    } ?>
  </body>
</html>
