<?php
function get_tasks($db, $day) {
    $sql = "SELECT days.day, tasknames.name, tasks.time_to_spend, tasks.time_already_spent, tasks.finished
        FROM days JOIN tasks ON days.id=tasks.days_id JOIN tasknames on tasks.tasknames_id=tasknames.id
        WHERE days.day=:day;";

    if (!($stmt = $db->prepare($sql))) {
        return [];
    }
    $stmt->bindParam(":day", $day);

    if (!$stmt->execute()) {
        return [];
    }
    $rows = $stmt->fetchAll(PDO::FETCH_ASSOC);
    return $rows;
}
$db = new PDO("sqlite:../database.sqlite3");
$day = $_POST["day"];
echo json_encode(get_tasks($db, $day));
?>
