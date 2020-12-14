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
        !isset($_POST["time_hours1"]) ||
        !isset($_POST["time_minutes1"]) ||
        $_POST["task1"] == "" ||
        $_POST["time_hours1"] == "" ||
        $_POST["time_minutes1"] == ""
    ) {
        die(
            "Please enter a task and time. Go <a href='../add_tasks.php'>Back</a>."
        );
    }
}

function get_tasks_and_times() {
    $tasks = [];
    $times = [];
    for ($i = 1; ; $i++) {
        if (!isset($_POST["task" . $i]) || !isset($_POST["time_hours" . $i]) || !isset($_POST["time_minutes" . $i])) {
            break;
        }
        $current_task = $_POST["task" . $i];
        $current_time_hours = $_POST["time_hours" . $i];
        $current_time_minutes = $_POST["time_minutes" . $i];
        if ($current_task !== "") {
            $tasks[] = $current_task;
        }
        if ($current_time_hours !== "" && $current_time_minutes !== "") {
            $current_time_minutes_total = $current_time_hours * 60 + $current_time_minutes;
            $times[] = $current_time_minutes_total;
        }
    }
    return ["tasks" => $tasks, "times" => $times];
}

function create_table_dates($db) {
    $sql = "CREATE TABLE IF NOT EXISTS dates (
        id INTEGER PRIMARY KEY,
        date TEXT NOT NULL UNIQUE
    );";

    if (!($stmt = $db->prepare($sql))) {
        die("Error in create_table_dates prepare: " . $db->error);
    }

    if (!$stmt->execute()) {
        print_r($stmt->errorInfo());
        die("Error in create_table_dates execute.");
    }
}

function create_table_tasks($db) {
    $sql = "CREATE TABLE IF NOT EXISTS tasks (
        id INTEGER PRIMARY KEY,
        minimum_time INTEGER NOT NULL DEFAULT 0,
        time_spent INTEGER NOT NULL DEFAULT 0,
        finished INTEGER NOT NULL DEFAULT 0,
        tasknames_id INTEGER NOT NULL,
        dates_id INTEGER NOT NULL
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
    create_table_dates($db);
    create_table_tasks($db);
    create_table_tasknames($db);
}

function insert_date($db, $date) {
    $sql = "INSERT OR IGNORE INTO dates (date) VALUES (?);";
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
    $sql = "INSERT INTO tasks (minimum_time, tasknames_id, dates_id)
        VALUES (:minimum_time, (SELECT id FROM tasknames WHERE name=:taskname), (SELECT id FROM dates WHERE date=:date));";
    if (!($stmt = $db->prepare($sql))) {
        die("Error in insert_task prepare: " . $db->error);
    }
    $stmt->bindParam(":minimum_time", $time);
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

header("Location: ../add_tasks.php?success=true");
exit();
?>
