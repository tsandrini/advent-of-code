open Core
open Utils

type direction = Left | Right

let direction_of_char = function
  | 'L' -> Left
  | 'R' -> Right
  | _ -> failwith "Invalid direction"

let solve lines =
  let line, lines = uncons lines in
  let directions = String.to_list line |> List.map ~f:direction_of_char in
  let maps = Hashtbl.create (module String) in
  let curr_ptr = ref "AAA" in
  let num_of_steps = ref 0 in
  (*Init hash table*)
  List.iter (List.drop lines 1) ~f:(fun line ->
      let key, values =
        String.filter line ~f:(fun c -> not (Char.is_whitespace c))
        |> String.split ~on:'=' |> UList.to_tuple_exn
      in
      let values =
        String.sub values ~pos:1 ~len:(String.length values - 2)
        |> String.split ~on:',' |> UList.to_tuple_exn
      in
      Hashtbl.add_exn maps ~key ~data:values);
  UList.repeat_until directions ~f:(fun direction ->
      let left_val, right_val = Hashtbl.find_exn maps !curr_ptr in
      (curr_ptr := match direction with Left -> left_val | Right -> right_val);
      num_of_steps := !num_of_steps + 1;
      String.(!curr_ptr = "ZZZ"));
  !num_of_steps

let main input =
  In_channel.read_lines input |> solve |> Printf.printf "Result: %d\n"

let%test "Day08part1 - test example input 1" =
  solve
    [ "LLR"; ""; "AAA = (BBB, BBB)"; "BBB = (AAA, ZZZ)"; "ZZZ = (ZZZ, ZZZ)" ]
  = 6

let%test "Day08part1 - test example input 2" =
  solve
    [
      "RL";
      "";
      "AAA = (BBB, CCC)";
      "BBB = (DDD, EEE)";
      "CCC = (ZZZ, GGG)";
      "DDD = (DDD, DDD)";
      "EEE = (EEE, EEE)";
      "GGG = (GGG, GGG)";
      "ZZZ = (ZZZ, ZZZ)";
    ]
  = 2
