<!DOCTYPE html>
<html>
  <head>
    <title>ProcStop - View Tasks</title>
    <meta charset="UTF-8" />
    <script src="js/main.js"></script>
	<link rel="stylesheet" href="css/main.css"/>
  </head>
  <body>
    <ul class="navbar">
      <li><a href="index.html">Home</a></li>
      <li><a href="add_tasks.php">Add Tasks</a></li>
      <li><a class="active" href="view_tasks.php">View Tasks</a></li>
    </ul>
	  <h1>View Tasks</h1>
        <form>
          <label for="date">Day:</label><br />
          <input type="date" id="date" name="date" value="<?php echo date("Y-m-d"); ?>" required/>
      <button type="button" onclick="set_task_table()">Get Tasks</button>
        </form><br>
    <table id="task_tbl" border="1">
    <tr>
        <th>Task</th>
        <th>Time</th>
        <th>Time Spent</th>
        <th>Finished</th>
    </tr>
    </table>
  </body>
</html>

