open Core
open Utils

let differences_of_list =
  let rec aux acc = function
    | [] -> acc
    | first :: second :: tl -> aux ((second - first) :: acc) (second :: tl)
    | [ _ ] -> acc
  in
  aux [] >> List.rev

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

let predict_prev_value histories =
  let curr_value = ref 0 in
  List.iter
    (List.drop (List.rev histories) 1)
    ~f:(fun history -> curr_value := List.hd_exn history - !curr_value);
  !curr_value

let parse =
  String.split_lines
  >> List.map
       ~f:(String.split ~on:' ' >> List.map ~f:Int.of_string >> history_of_list)

let part1 = List.map ~f:predict_next_value >> UList.fold_sum
let part2 = List.map ~f:predict_prev_value >> UList.fold_sum

let solve processed_inp =
  Printf.printf "Part 1: %d\n" (part1 processed_inp);
  Printf.printf "Part 2: %d\n" (part2 processed_inp)

let main = In_channel.read_all >> parse >> solve

let%test "Day09 part1 - example data" =
  (parse >> part1) "0 3 6 9 12 15\n1 3 6 10 15 21\n10 13 16 21 30 45" = 114

let%test "Day09 part2 - example data" =
  (parse >> part2) "0 3 6 9 12 15\n1 3 6 10 15 21\n10 13 16 21 30 45" = 2
