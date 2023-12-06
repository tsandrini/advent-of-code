(* Utop line *)
(* #use "topfind";; #require "core";; #require "ppx_jane";; #require "ppx_deriving";; #require "ppx_deriving.std";; #require "ppx_inline_test";; #require "ppx_hash";; #require "ppx_sexp_conv";; #require "ppx_compare";; *)

module List' = List
module String' = String
open Core

let solve lines =
  let parseLine line =
    line |> Utils.reduce_multiple_whitespaces |> String.split ~on:':'
    |> List'.tl |> List'.hd |> String.strip |> String.split ~on:' '
    |> List.map ~f:Int.of_string
  in
  let times = parseLine (List'.nth lines 0) in
  let distances = parseLine (List'.nth lines 1) in
  List.map2_exn times distances ~f:(fun time dist ->
      List.range 0 time
      |> List.filter ~f:(fun hold_time -> hold_time * (time - hold_time) > dist)
      |> List.length)
  |> List.fold_left ~init:1 ~f:(fun acc x -> acc * x)

let main input =
  In_channel.read_lines input |> solve |> Printf.printf "Result: %d\n"

let%test "Day06part1 - test example input" =
  solve [ "Time:      7  15   30"; "Distance:  9  40  200" ] = 288
