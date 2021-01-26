<!DOCTYPE html>
<html>
  <head>
    <title>ProcStop - Stats</title>
    <link rel="shortcut icon" type="image/png" href="favicon.png"/>
    <link rel="shortcut icon" sizes="196x196" type="image/png" href="icon196.png"/>
    <meta charset="UTF-8" />
	<meta name="viewport" content="width=device-width, initial-scale=1.0">
    <script src="js/main.js"></script>
	<link rel="stylesheet" href="css/main.css"/>
  </head>
  <body>
    <ul class="navbar">
      <li><a href="index.html">Home</a></li>
      <li><a href="add_tasks.php">Add Tasks</a></li>
      <li><a href="view_tasks.php">View Tasks</a></li>
      <li><a class="active" href="stats.php">Stats</a></li>
    </ul>
	  <h1>Stats</h1>
        <form>
          <label for="task">Task:</label><br />
	<select name="task" id="task_select">
	  <option value="<all>">All Tasks</option>
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
                echo "<option value='" . $row["name"] . "'>" . $row["name"] . "</option>";
            }
	?>
	</select><br><br>
          <label for="time">Time:</label><br />
	<select name="time" id="time_select">
	  <option value="<all>">All Time</option>
	  <option value="<week>">Past Week</option>
	  <option value="<month>">Past Month</option>
	  <option value="<year>">Past Year</option>
	</select><br>
      <button type="button" onclick="get_stats()">Get Stats</button>
      <div id="stats_results"></div>
  </body>
</html>
