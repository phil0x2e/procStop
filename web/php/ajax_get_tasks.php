<?php
function get_tasks($db, $date) {
    $sql = "SELECT dates.date, tasknames.name, tasks.minimum_time, tasks.time_spent, tasks.finished
        FROM dates JOIN tasks ON dates.id=tasks.dates_id JOIN tasknames on tasks.tasknames_id=tasknames.id
        WHERE dates.date=:date;";

    if (!($stmt = $db->prepare($sql))) {
        return [];
    }
    $stmt->bindParam(":date", $date);

    if (!$stmt->execute()) {
        return [];
    }
    $rows = $stmt->fetchAll(PDO::FETCH_ASSOC);
    return $rows;
}
$db = new PDO("sqlite:../database.sqlite3");
$date = $_POST["date"];
echo json_encode(get_tasks($db, $date));
?>
