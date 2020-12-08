<?php
ini_set("display_errors", 1);
ini_set("track_errors", 1);
ini_set("html_errors", 1);
error_reporting(E_ALL);

function check() {
    if (!isset($_POST["date"]) || $_POST["date"] == "") {
        die("Please select date. Go <a href='../add_tasks.php'>Back</a>.");
    }
    if (
        !isset($_POST["task1"]) ||
        !isset($_POST["time1"]) ||
        $_POST["task1"] == "" ||
        $_POST["time1"] == ""
    ) {
        die(
            "Please enter a task with a time. Go <a href='../add_tasks.php'>Back</a>."
        );
    }
}

function get_tasks_and_times() {
    $tasks = [];
    $times = [];
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
    return ["tasks" => $tasks, "times" => $times];
}

function create_table_days($db) {
    $sql = "CREATE TABLE IF NOT EXISTS days (
        id INTEGER PRIMARY KEY,
        day TEXT NOT NULL UNIQUE
    );";

    if (!($stmt = $db->prepare($sql))) {
        die("Error in create_table_days prepare: " . $db->error);
    }

    if (!$stmt->execute()) {
        print_r($stmt->errorInfo());
        die("Error in create_table_days execute.");
    }
}

function create_table_tasks($db) {
    $sql = "CREATE TABLE IF NOT EXISTS tasks (
        id INTEGER PRIMARY KEY,
        time_to_spend INTEGER NOT NULL DEFAULT 0,
        time_already_spent INTEGER NOT NULL DEFAULT 0,
        finished INTEGER NOT NULL DEFAULT 0,
        tasknames_id INTEGER NOT NULL,
        days_id INTEGER NOT NULL
    );";

    if (!($stmt = $db->prepare($sql))) {
        die("Error in create_table_tasks prepare: " . $db->error);
    }

    if (!$stmt->execute()) {
        print_r($stmt->errorInfo());
        die("Error in create_table_tasks execute.");
    }
}

function create_table_tasknames($db) {
    $sql = "CREATE TABLE IF NOT EXISTS tasknames (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL UNIQUE
    );";

    if (!($stmt = $db->prepare($sql))) {
        die("Error in create_table_tasknames prepare: " . $db->error);
    }

    if (!$stmt->execute()) {
        print_r($stmt->errorInfo());
        die("Error in create_table_tasknames execute.");
    }
}

function create_tables($db) {
    create_table_days($db);
    create_table_tasks($db);
    create_table_tasknames($db);
}

function insert_date($db, $date) {
    $sql = "INSERT OR IGNORE INTO days (day) VALUES (?);";
    if (!($stmt = $db->prepare($sql))) {
        die("Error in insert_date prepare: " . $db->error);
    }
    $stmt->bindParam(1, $date);
    if (!$stmt->execute()) {
        print_r($stmt->errorInfo());
        die("Error in insert_date execute.");
    }
}

function insert_taskname($db, $taskname) {
    $sql = "INSERT OR IGNORE INTO tasknames (name) VALUES (?);";
    if (!($stmt = $db->prepare($sql))) {
        die("Error in insert_taskname prepare: " . $db->error);
    }
    $stmt->bindParam(1, $taskname);
    if (!$stmt->execute()) {
        print_r($stmt->errorInfo());
        die("Error in insert_taskname execute.");
    }
}

function insert_task($db, $taskname, $time, $date) {
    $sql = "INSERT INTO tasks (time_to_spend, tasknames_id, days_id)
        VALUES (:time_to_spend, (SELECT id FROM tasknames WHERE name=:taskname), (SELECT id FROM days WHERE day=:date));";
    if (!($stmt = $db->prepare($sql))) {
        die("Error in insert_task prepare: " . $db->error);
    }
    $stmt->bindParam(":time_to_spend", $time);
    $stmt->bindParam(":taskname", $taskname);
    $stmt->bindParam(":date", $date);
    if (!$stmt->execute()) {
        print_r($stmt->errorInfo());
        die("Error in insert_task execute.");
    }
}

function insert_all($db, $date, $tasks_and_times) {
    insert_date($db, $date);
    foreach ($tasks_and_times["tasks"] as $key => $taskname) {
        $time = $tasks_and_times["times"][$key];
        insert_taskname($db, $taskname);
        insert_task($db, $taskname, $time, $date);
    }
}

check();
$date = $_POST["date"];
$tasks_and_times = get_tasks_and_times();
$db = new PDO("sqlite:../database.sqlite3");
create_tables($db);
insert_all($db, $date, $tasks_and_times);

header("Location: ../index.html");
exit();
?>
