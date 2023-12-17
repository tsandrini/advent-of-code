open Core
open Utils

type direction = E | W | N | S

let num_cycles = 1000000000

let compute_north_load grid =
  List.foldi grid ~init:0 ~f:(fun idx acc row ->
      acc + ((idx + 1) * List.count row ~f:(fun c -> Char.(c = 'O'))))

let roll_rocks_east grid =
  List.map grid ~f:(fun row ->
      String.of_char_list row |> String.split ~on:'#'
      |> List.map ~f:(fun part ->
             let len = String.length part in
             let rocks_count = String.count part ~f:(fun c -> Char.(c = 'O')) in
             String.init (len - rocks_count) ~f:(fun _ -> '.')
             ^ String.init rocks_count ~f:(fun _ -> 'O'))
      |> String.concat ~sep:"#" |> String.to_list)

let roll_rocks grid = function
  | E -> roll_rocks_east grid
  | W -> List.map grid ~f:List.rev |> roll_rocks_east |> List.map ~f:List.rev
  | N -> UMat.rot_90cw grid |> roll_rocks_east |> UMat.rot_90ccw
  | S -> UMat.transpose grid |> roll_rocks_east |> UMat.transpose

let one_cycle grid =
  ((roll_rocks grid N |> fun g -> roll_rocks g W) |> fun g -> roll_rocks g S)
  |> fun g -> roll_rocks g E

let loop_until_cycle initial_grid =
  let memo = Hashtbl.Poly.create () in
  let rec aux idx grid_state =
    let next_grid = one_cycle grid_state in
    match Hashtbl.find memo next_grid with
    | Some start_idx -> (start_idx, idx, next_grid)
    | None ->
        Hashtbl.add_exn memo ~key:next_grid ~data:idx;
        aux (idx + 1) next_grid
  in
  aux 0 initial_grid

let parse input = String.split_lines input |> List.map ~f:String.to_list

let solve grid =
  let part1 = roll_rocks grid N |> List.rev |> compute_north_load in
  let cycle_start, cycle_end, grid_cycle_state = loop_until_cycle grid in
  let final_grid_state =
    apply_n_times one_cycle
      ~n:(((num_cycles - cycle_start) % (cycle_end - cycle_start)) - 1)
      grid_cycle_state
  in
  let part2 = final_grid_state |> List.rev |> compute_north_load in
  Printf.printf "Part1 - %d\n" part1;
  Printf.printf "Part2 - %d\n" part2

let main input = In_channel.read_all input |> parse |> solve
