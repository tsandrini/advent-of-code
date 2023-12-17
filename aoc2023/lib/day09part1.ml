open Core
open Utils

let differences_of_list lst =
  let rec aux acc = function
    | [] -> acc
    | first :: second :: tl -> aux ((second - first) :: acc) (second :: tl)
    | [ _ ] -> acc
  in
  aux [] lst |> List.rev

let history_of_list lst =
  let histories = ref [ lst ] in
  let curr_history = ref lst in
  while
    List.fold_left ~init:false ~f:(fun acc el -> acc || el <> 0) !curr_history
  do
    curr_history := differences_of_list !curr_history;
    histories := !curr_history :: !histories
  done;
  !histories |> List.rev

let predict_next_value histories =
  let curr_value = ref 0 in
  List.iter
    (List.drop (List.rev histories) 1)
    ~f:(fun history -> curr_value := !curr_value + List.last_exn history);
  !curr_value

let solve lines =
  List.map lines ~f:(fun line ->
      String.split line ~on:' ' |> List.map ~f:Int.of_string |> history_of_list
      |> predict_next_value)
  |> UList.fold_sum

let main input =
  In_channel.read_lines input |> solve |> Printf.printf "Result: %d\n"

let%test "Day09part1 - test example input" =
  solve [ "0 3 6 9 12 15"; "1 3 6 10 15 21"; "10 13 16 21 30 45" ] = 114
