<?php
function delete_task($db, $task_id) {
    $sql = "DELETE FROM tasks WHERE id=:task_id;";

    if (!($stmt = $db->prepare($sql))) {
        return [];
    }
    $stmt->bindParam(":task_id", $task_id);

    if (!$stmt->execute()) {
        return [];
    }
}
$db = new PDO("sqlite:../database.sqlite3");
$task_id = $_POST["task_id"];
delete_task($db, $task_id);
?>
