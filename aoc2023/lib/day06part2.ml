open Core
open Utils

let solve lines =
  let parseLine line =
    line |> UString.reduce_multiple_whitespaces |> String.split ~on:':'
    |> List.tl_exn |> List.hd_exn |> String.strip |> String.split ~on:' '
    |> List.fold_left ~f:( ^ ) ~init:""
    |> Int.of_string
  in
  let time = parseLine (List.nth_exn lines 0) in
  let dist = parseLine (List.nth_exn lines 1) in
  List.range 0 time
  |> List.filter ~f:(fun hold_time -> hold_time * (time - hold_time) > dist)
  |> List.length

let main input =
  In_channel.read_lines input |> solve |> Printf.printf "Result: %d\n"

let%test "Day06part2 - test example input" =
  solve [ "Time:      7  15   30"; "Distance:  9  40  200" ] = 71503
