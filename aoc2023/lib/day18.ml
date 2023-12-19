open Core
open Utils

let dig_in_dir ~dir ~steps (x, y) =
  match dir with
  | 'U' -> (x, y - steps)
  | 'D' -> (x, y + steps)
  | 'L' -> (x - steps, y)
  | 'R' -> (x + steps, y)
  | _ -> (x, y)

let dir_of_num = function
  | '0' -> 'R'
  | '1' -> 'D'
  | '2' -> 'L'
  | '3' -> 'U'
  | _ -> failwith "Invalid number"

let shoelace_area points =
  let rec aux acc points =
    match points with
    | (x1, y1) :: ((x2, y2) :: _ as rest) ->
        aux (acc + ((x1 * y2) - (x2 * y1))) rest
    | _ -> acc
  in
  let closed_points = points @ [ List.hd_exn points ] in
  Int.abs (aux 0 closed_points) / 2

let traverse_grid ?(start_pos = (0, 0)) ?(use_hex = false) lines =
  let cycle = ref [] in
  let cycle_len = ref 0 in
  let curr_pos = ref start_pos in
  List.iter lines ~f:(fun (dir, num, hex) ->
      let target_num =
        if use_hex then Int.of_string ("0x" ^ String.sub ~pos:0 ~len:5 hex)
        else num
      in
      let target_dir =
        if use_hex then
          String.sub ~pos:5 ~len:1 hex |> Char.of_string |> dir_of_num
        else dir
      in

      curr_pos := dig_in_dir ~dir:target_dir ~steps:target_num !curr_pos;
      cycle := !curr_pos :: !cycle;
      cycle_len := !cycle_len + target_num);
  (start_pos :: !cycle, !cycle_len)

let parse =
  String.split_lines
  >> List.map ~f:(fun line ->
         let dir, num, color =
           String.split ~on:' ' line |> UList.to_triple_exn
         in
         ( Char.of_string dir,
           Int.of_string num,
           String.drop_prefix (String.drop_suffix color 1) 2 ))

let part1 lines =
  let visited, cycle_len = traverse_grid ~use_hex:false lines in
  let cycle_integral = shoelace_area visited - (cycle_len / 2) + 1 in
  cycle_integral + cycle_len

let part2 lines =
  let visited, cycle_len = traverse_grid ~use_hex:true lines in
  let cycle_integral = shoelace_area visited - (cycle_len / 2) + 1 in
  cycle_integral + cycle_len

let solve processed_inp =
  Printf.printf "Part 1: %d\n" (part1 processed_inp);
  Printf.printf "Part 2: %d\n" (part2 processed_inp)

let main = In_channel.read_all >> parse >> solve

let%test "Day18 part1 - example data" =
  (parse >> part1)
    "R 6 (#70c710)\n\
     D 5 (#0dc571)\n\
     L 2 (#5713f0)\n\
     D 2 (#d2c081)\n\
     R 2 (#59c680)\n\
     D 2 (#411b91)\n\
     L 5 (#8ceee2)\n\
     U 2 (#caa173)\n\
     L 1 (#1b58a2)\n\
     U 2 (#caa171)\n\
     R 2 (#7807d2)\n\
     U 3 (#a77fa3)\n\
     L 2 (#015232)\n\
     U 2 (#7a21e3)"
  = 62

let%test "Day18 part2 - example data" =
  (parse >> part2)
    "R 6 (#70c710)\n\
     D 5 (#0dc571)\n\
     L 2 (#5713f0)\n\
     D 2 (#d2c081)\n\
     R 2 (#59c680)\n\
     D 2 (#411b91)\n\
     L 5 (#8ceee2)\n\
     U 2 (#caa173)\n\
     L 1 (#1b58a2)\n\
     U 2 (#caa171)\n\
     R 2 (#7807d2)\n\
     U 3 (#a77fa3)\n\
     L 2 (#015232)\n\
     U 2 (#7a21e3)"
  = 952408144115
