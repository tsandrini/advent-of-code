module List' = List
module String' = String
open Core

type direction = Left | Right

let direction_of_char = function
  | 'L' -> Left
  | 'R' -> Right
  | _ -> failwith "Invalid direction"

let solve lines =
  let line, lines = Utils.uncons lines in
  let directions = String.to_list line |> List.map ~f:direction_of_char in
  let maps = Hashtbl.create (module String) in
  let num_of_steps_curr = ref 0 in
  let nums_of_steps = ref [] in
  let starting_positions = ref [] in
  let curr_ptr = ref "" in
  (*Init hash table*)
  List.iter (List.drop lines 1) ~f:(fun line ->
      let key, values =
        String.filter line ~f:(fun c -> not (Char.is_whitespace c))
        |> String.split ~on:'=' |> Utils.list_to_tuple
        |> fun (k, v) ->
        ( k,
          String.sub v ~pos:1 ~len:(String.length v - 2)
          |> String.split ~on:',' |> Utils.list_to_tuple )
      in
      if Char.(String.get key 2 = 'A') then
        starting_positions := key :: !starting_positions;

      Hashtbl.add_exn maps ~key ~data:values);
  (*For each starting position, iterate until we find 'Z' in the last pos and *)
  (*save counter -> afterwards fold over lcm on the results*)
  List.iter !starting_positions ~f:(fun starting_position ->
      num_of_steps_curr := 0;
      curr_ptr := starting_position;
      Utils.list_repeat_until directions ~f:(fun direction ->
          let left_val, right_val = Hashtbl.find_exn maps !curr_ptr in
          (curr_ptr :=
             match direction with Left -> left_val | Right -> right_val);
          num_of_steps_curr := !num_of_steps_curr + 1;
          Char.(String.get !curr_ptr 2 = 'Z'));
      nums_of_steps := !num_of_steps_curr :: !nums_of_steps);
  List.fold_left ~init:1 ~f:Utils.lcm !nums_of_steps

let main input =
  In_channel.read_lines input |> solve |> Printf.printf "Result: %d\n"
