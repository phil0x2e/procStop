<?php
function get_stats_task_name($db, $task_name, $time) {
    $sql = "";
    if ($time == "<week>") {
        $sql = "SELECT dates.date, tasknames.name, tasks.minimum_time, tasks.time_spent, tasks.finished
    	FROM dates JOIN tasks ON dates.id=tasks.dates_id JOIN tasknames on tasks.tasknames_id=tasknames.id
    	WHERE tasknames.name=:task_name AND dates.date > DATE('now', '-7 day');";
    }
    elseif ($time == "<month>") {
        $sql = "SELECT dates.date, tasknames.name, tasks.minimum_time, tasks.time_spent, tasks.finished
    	FROM dates JOIN tasks ON dates.id=tasks.dates_id JOIN tasknames on tasks.tasknames_id=tasknames.id
    	WHERE tasknames.name=:task_name AND dates.date > DATE('now', '-1 month');";
    }
    elseif ($time == "<year>") {
        $sql = "SELECT dates.date, tasknames.name, tasks.minimum_time, tasks.time_spent, tasks.finished
    	FROM dates JOIN tasks ON dates.id=tasks.dates_id JOIN tasknames on tasks.tasknames_id=tasknames.id
    	WHERE tasknames.name=:task_name AND dates.date > DATE('now', '-1 year');";
    }
    else {
        $sql = "SELECT dates.date, tasknames.name, tasks.minimum_time, tasks.time_spent, tasks.finished
	FROM dates JOIN tasks ON dates.id=tasks.dates_id JOIN tasknames on tasks.tasknames_id=tasknames.id
	WHERE tasknames.name=:task_name;";
    }

    if (!($stmt = $db->prepare($sql))) {
        return [];
    }
    $stmt->bindParam(":task_name", $task_name);

    if (!$stmt->execute()) {
        return [];
    }
    $rows = $stmt->fetchAll(PDO::FETCH_ASSOC);
    return $rows;
}

function get_stats($db, $time) {
    $sql = "";
    if ($time == "<week>") {
        $sql = "SELECT dates.date, tasknames.name, tasks.minimum_time, tasks.time_spent, tasks.finished
    	FROM dates JOIN tasks ON dates.id=tasks.dates_id JOIN tasknames on tasks.tasknames_id=tasknames.id
    	WHERE dates.date > DATE('now', '-7 day');";
    }
    elseif ($time == "<month>") {
        $sql = "SELECT dates.date, tasknames.name, tasks.minimum_time, tasks.time_spent, tasks.finished
    	FROM dates JOIN tasks ON dates.id=tasks.dates_id JOIN tasknames on tasks.tasknames_id=tasknames.id
    	WHERE dates.date > DATE('now', '-1 month');";
    }
    elseif ($time == "<year>") {
        $sql = "SELECT dates.date, tasknames.name, tasks.minimum_time, tasks.time_spent, tasks.finished
    	FROM dates JOIN tasks ON dates.id=tasks.dates_id JOIN tasknames on tasks.tasknames_id=tasknames.id
    	WHERE dates.date > DATE('now', '-1 year');";
    }
    else {
        $sql = "SELECT dates.date, tasknames.name, tasks.minimum_time, tasks.time_spent, tasks.finished
    	FROM dates JOIN tasks ON dates.id=tasks.dates_id JOIN tasknames on tasks.tasknames_id=tasknames.id;";
    }
    if (!($stmt = $db->prepare($sql))) {
        return [];
    }

    if (!$stmt->execute()) {
        return [];
    }
    $rows = $stmt->fetchAll(PDO::FETCH_ASSOC);
    return $rows;
}

$db = new PDO("sqlite:../database.sqlite3");
$task_name = $_POST["task_name"];
$time = $_POST["time"];

if ($task_name == "<all>") {
    echo json_encode(get_stats($db, $time));
}else {
    echo json_encode(get_stats_task_name($db, $task_name, $time));
}
?>
