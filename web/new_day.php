<?php
$tasks = [];
$times = [];
for ($i = 1; ; $i++) {
    if (!isset($_POST["task" . $i]) || !isset($_POST["time" . $i])) {
        break;
    }
    $tasks[] = $_POST["task" . $i];
    $times[] = $_POST["time" . $i];
}
?>
