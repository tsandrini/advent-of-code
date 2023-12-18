open Core
open Utils

let parse = String.split_lines

let part1 = List.length
let part2 = List.length

let solve processed_inp =
  Printf.printf "Part 1: %d\n" (part1 processed_inp);
  Printf.printf "Part 2: %d\n" (part2 processed_inp)

let main = In_channel.read_all >> parse >> solve

(* let%test "Day16 part1 - example data" = *)
(*   (parse >> part1) *)
(*     "" *)
(*   = 46 *)

(* let%test "Day16 part2 - example data" = *)
(*   (parse >> part2) *)
(*     "" *)
(*   = 51 *)
