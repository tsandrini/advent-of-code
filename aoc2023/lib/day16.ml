open Core
open Utils

module IntPair = struct
  type t = int * int [@@deriving sexp, compare, hash]
end

let get_neighbors grid (x, y) (dx, dy) =
  let m e d =
    match Hashtbl.find grid e with Some _ -> [ (e, d) ] | None -> []
  in
  match Hashtbl.find grid (x, y) with
  | Some '.' -> m (x + dx, y + dy) (dx, dy)
  | Some '|' ->
      if dx = 0 then m (x, y + dy) (dx, dy)
      else m (x, y + 1) (0, 1) @ m (x, y - 1) (0, -1)
  | Some '-' ->
      if dy = 0 then m (x + dx, y) (dx, dy)
      else m (x + 1, y) (1, 0) @ m (x - 1, y) (-1, 0)
  | Some '\\' -> m (x + dy, y + dx) (dy, dx)
  | Some '/' -> m (x - dy, y - dx) (-dy, -dx)
  | _ -> []

let traverse ?(start_pos = ((0, 0), (1, 0))) grid =
  let visited = Hashtbl.Poly.create () in
  let energized = Hashtbl.create (module IntPair) in
  let queue = Linked_queue.create () in

  Linked_queue.enqueue queue start_pos;
  while not (Linked_queue.is_empty queue) do
    let node, derivative = Linked_queue.dequeue_exn queue in
    if not (Hashtbl.mem visited (node, derivative)) then (
      Hashtbl.set visited ~key:(node, derivative) ~data:();
      Hashtbl.set energized ~key:node ~data:();
      List.iter (get_neighbors grid node derivative)
        ~f:(fun (node', derivative') ->
          if not (Hashtbl.mem visited (node', derivative')) then
            Linked_queue.enqueue queue (node', derivative')))
  done;
  Hashtbl.fold energized ~init:0 ~f:(fun ~key:_ ~data:_ acc -> acc + 1)

let parse input =
  let grid = Hashtbl.create (module IntPair) in
  let height = String.split_lines input |> List.length in
  let width = String.split_lines input |> List.hd_exn |> String.length in
  String.split_lines input
  |> List.iteri ~f:(fun y line ->
         String.to_list line
         |> List.iteri ~f:(fun x c -> Hashtbl.set grid ~key:(x, y) ~data:c));
  (grid, (width, height))

let part1 (grid, _) = traverse grid

let part2 (grid, (width, height)) =
  [
    List.init width ~f:(fun x -> ((x, 0), (0, 1)));
    List.init width ~f:(fun x -> ((x, height - 1), (0, -1)));
    List.init height ~f:(fun y -> ((0, y), (1, 0)));
  ]
  |> List.concat
  |> List.map ~f:(fun start_pos -> traverse ~start_pos grid)
  |> List.max_elt ~compare:Int.compare
  |> option_unwrap

let solve processed_inp =
  Printf.printf "Part 1: %d\n" (part1 processed_inp);
  Printf.printf "Part 2: %d\n" (part2 processed_inp)

let main = In_channel.read_all >> parse >> solve

let%test "Day16 part1 - example data" =
  (parse >> part1)
    ".|...\\....\n\
     |.-.\\.....\n\
     .....|-...\n\
     ........|.\n\
     ..........\n\
     .........\\\n\
     ..../.\\\\..\n\
     .-.-/..|..\n\
     .|....-|.\\\n\
     ..//.|...."
  = 46

let%test "Day16 part2 - example data" =
  (parse >> part2)
    ".|...\\....\n\
     |.-.\\.....\n\
     .....|-...\n\
     ........|.\n\
     ..........\n\
     .........\\\n\
     ..../.\\\\..\n\
     .-.-/..|..\n\
     .|....-|.\\\n\
     ..//.|...."
  = 51
