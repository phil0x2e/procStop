<?php
$tasks = [];
$times = [];

if (!isset($_POST["date"])){
   die("Please select date.");
}
if (!isset($_POST["task1"]) || !isset($_POST["time1"]) || $_POST["task1"] == "" || $_POST["time1"] == ""){
    die("Please enter a task with a time.");
}

$date = $_POST["date"];
for ($i = 1; ; $i++) {
    if (!isset($_POST["task" . $i]) || !isset($_POST["time" . $i])) {
        break;
    }
    $current_task = $_POST["task" . $i];
    $current_time = $_POST["time" . $i];
    if ($current_task !== "") {
        $tasks[] = $current_task;
    }
    if ($current_time !== "") {
        $times[] = $current_time;
    }
}
$sql = "CREATE TABLE IF NOT EXISTS days (
	id INTEGER PRIMARY KEY,
	day TEXT NOT NULL
);"

$db = new PDO("sqlite:database.sqlite3");
if (!($stmt = $db->prepare($sql))) {
	die("Error in prepare: ".$db->error);
}
	
if (!$stmt->execute()){
	die("Error in execute: ".$stmt->error);
}
$stmt->close();
?>
