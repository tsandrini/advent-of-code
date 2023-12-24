open Core
open Utils

type direction = Down | Left | Right | Up [@@deriving sexp, compare, hash]
type node = int * int [@@deriving sexp, compare, hash]
type extended_node = { pos: node; last_move: direction; consecutive_moves: int } [@@deriving sexp, compare, hash]

module ExtendedNodeTable = Hashtbl.Make (struct
    type t = extended_node
    let t_of_sexp = extended_node_of_sexp
    let sexp_of_t = sexp_of_extended_node
    let hash en = Hashtbl.hash (en.pos)
    let equal en1 en2 =
      let (x1, y1) = en1.pos and (x2, y2) = en2.pos in
      x1 = x2 && y1 = y2
    let compare en1 en2 = if equal en1 en2 then 0 else compare_node en1.pos en2.pos
  end)

let print_path ~width ~height path =
  let grid = Array.make_matrix ~dimy:height ~dimx:width '.' in

  List.iter ~f:(fun (x, y) -> grid.(y).(x) <- '#') path;

  for y = 0 to height - 1 do
    for x = 0 to width - 1 do
      Printf.printf "%c" grid.(y).(x)
    done;
    Printf.printf "\n"
  done


let manhattan_dist (x1, y1) (x2, y2) = abs (x1 - x2) + abs (y1 - y2)

let at ~dim:(width, _) grid (x, y) =
  grid.(y * width + x)

let print_path_vals ~gr ~width ~height path =
  let grid = Array.make_matrix ~dimy:height ~dimx:width '.' in
  let f = at ~dim:(width, height) gr in
  let out = ref 0 in

  List.iter ~f:(fun (x, y) -> grid.(y).(x) <- (f (x, y) |> Int.to_string |> Char.of_string); out := !out + (f (x,y))) path;

  for y = 0 to height - 1 do
    for x = 0 to width - 1 do
      Printf.printf "%c" grid.(y).(x)
    done;
    Printf.printf "\n"
  done;
  Printf.printf "out: %d\n" !out

let generate_neighbors ~dim:(width, height) ({pos = (x, y); last_move; consecutive_moves}) =
  let moves = match last_move with | Up -> [Up; Left; Right] | Down -> [Down; Left; Right] | Left -> [Left; Up; Down] | Right -> [Right; Up; Down]
  and is_ouf_of_bounds en = match en.pos with
    | (x, y) -> x < 0 || x >= width || y < 0 || y >= height
  in List.filter_map ~f:(fun move ->
    if (compare_direction move last_move) = 0 && consecutive_moves >= 3 then
      None (* Skip this move as the limit is reached *)
    else
      let new_position = match move with
        | Up -> (x, y - 1)
        | Down -> (x, y + 1)
        | Left -> (x - 1, y)
        | Right -> (x + 1, y)
      in
      let new_consecutive_moves = if (compare_direction move last_move) = 0 then consecutive_moves + 1 else 1 in
      Some {pos = new_position; last_move = move; consecutive_moves = new_consecutive_moves}
  ) moves |> List.filter ~f:(fun en -> not (is_ouf_of_bounds en))

let h_of_grid ~dim ~grid (_, _) (x2, y2) =
  at ~dim grid (x2, y2)

let astar ~dim:(width, height) ~start_pos ~goal_pos ~heuristic ~cost _ =
  let open_set = Pairing_heap.create ~cmp:(fun (_, c1) (_, c2) -> Int.compare c1 c2) ()
  and closed_set = ExtendedNodeTable.create ~size:(10 * width * height) ()
  and g_score = ExtendedNodeTable.create ~size:(10 * width * height) ()
  and came_from = ExtendedNodeTable.create ~size:(10 * width * height) () in

  let start_node = { pos = start_pos; last_move = Left; consecutive_moves = -1 } in
  ExtendedNodeTable.add_exn g_score ~key:start_node ~data:0;
  Pairing_heap.add open_set (start_node, 0);

  let rec reconstruct_path current acc =
    match ExtendedNodeTable.find came_from current with
    | None -> List.rev acc
    | Some prev -> reconstruct_path prev (current.pos :: acc)
  in

  let rec main_loop () =
    if not (Pairing_heap.is_empty open_set) then
      let curr, current_cost = Pairing_heap.pop_exn open_set in

      if (compare_node curr.pos goal_pos) = 0 then
        Some (reconstruct_path curr [], ExtendedNodeTable.find_exn g_score curr)
      else begin
        generate_neighbors ~dim:(width, height) curr
        |> List.iter ~f:(fun neighbor ->
            if not (ExtendedNodeTable.mem closed_set neighbor) then
              let tentative_g_score = current_cost + cost curr.pos neighbor.pos in

              let is_better_path = match ExtendedNodeTable.find g_score neighbor with
                | None -> true
                | Some existing_g_score -> tentative_g_score < existing_g_score
              in

              if is_better_path then begin
                ExtendedNodeTable.set came_from ~key:neighbor ~data:curr;
                ExtendedNodeTable.set g_score ~key:neighbor ~data:tentative_g_score;
                let total_cost = tentative_g_score + heuristic neighbor.pos goal_pos in
                Pairing_heap.add open_set (neighbor, total_cost)
              end
          );

        if not (ExtendedNodeTable.mem closed_set curr) then
          ExtendedNodeTable.add_exn closed_set ~key:curr ~data:true;
        main_loop ()
      end
    else
      None
  in
  main_loop ()

let parse input =
  let lines = String.split_lines input |> List.map ~f:(String.to_list >> List.map ~f:(Char.to_string >> Int.of_string)) in
  let height = List.length lines in
  let width = List.length (List.hd_exn lines) in
  (List.concat lines |> Array.of_list, (width, height))

let part1 (grid, (width, height)) =
  let start_pos = (0, 0)
  and goal_pos = (width - 1, height - 1)
  and cost = h_of_grid ~dim:(width, height) ~grid
  and heuristic _ _ = 0
  in match astar ~dim:(width, height) ~start_pos ~goal_pos ~cost ~heuristic grid with
  | Some (path, cost) -> Printf.printf "Path: %s\n" (List.map path ~f:(fun (x, y) -> sprintf "(%d, %d)" x y) |> String.concat ~sep:" -> "); print_path ~width ~height path; print_path_vals ~gr:grid ~width ~height path; cost
  | None -> failwith "No path found"

let part2 (grid, _) = Array.length grid

let solve processed_inp =
  Printf.printf "Part 1: %d\n" (part1 processed_inp);
  Printf.printf "Part 2: %d\n" (part2 processed_inp)

let main = In_channel.read_all >> parse >> solve

let%test "Day17 part1 - example data" =
  (parse >> part1)
    "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"
  = 102

(* let%test "Day16 part2 - example data" = *)
(*   (parse >> part2) *)
(*     "" *)
(*   = 51 *)
