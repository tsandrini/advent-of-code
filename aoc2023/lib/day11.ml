open Core

let int_node_type_of_char = function
  | '.' -> 0
  | '#' -> 1
  | _ -> failwith "Unknown node type"

let parse input =
  let transpose mat =
    let height = List.length mat in
    let width = List.length (List.hd_exn mat) in
    Array.init width ~f:(fun y ->
        Array.init height ~f:(fun x ->
            List.nth_exn (List.nth_exn mat x) y)) |> Array.map ~f:Array.to_list |> Array.to_list in
  let space =
    String.split_lines input
    |> List.map ~f:(fun line ->
           String.to_list line
           |> List.map ~f:int_node_type_of_char) in
  let empty_rows = List.mapi space ~f:(fun i row -> if (List.fold_left ~init:0 ~f:(+) row) = 0 then i else -1) |> List.filter ~f:(fun y -> y <> -1) in
  let empty_cols = transpose space |> List.mapi ~f:(fun i row -> if (List.fold_left ~init:0 ~f:(+) row) = 0 then i else -1) |> List.filter ~f:(fun x -> x <> -1) in
  Printf.printf "Empty_rows = %s\n" (List.to_string empty_rows ~f:Int.to_string);
  Printf.printf "Empty_cols = %s\n" (List.to_string empty_cols ~f:Int.to_string);
  let galaxies = ref [] in
  List.iteri ~f:(fun y row ->
      List.iteri ~f:(fun x node ->
          if node = 1 then galaxies := (x, y) :: !galaxies;) row) space;
  (space, !galaxies, (empty_rows, empty_cols))

let manhattan_dist (x1, y1) (x2, y2) = abs (x1 - x2) + abs (y1 - y2)

let compute_distances ?(expansion_rate=2) ~galaxies ~empty:(empty_rows, empty_cols) =
  let expansion_shift_in (x1, y1) (x2, y2) =
    let xmin, xmax = (min x1 x2, max x1 x2) in
    let ymin, ymax = (min y1 y2, max y1 y2) in
    let cols_in = List.filter empty_cols ~f:(fun x -> x > xmin && x < xmax) in
    let rows_in = List.filter empty_rows ~f:(fun y -> y > ymin && y < ymax) in
    (* Printf.printf "Checking galaxies (%d, %d) and (%d, %d) - cols_in: %d, rows_in: %d\n" x1 y1 x2 y2 (List.length cols_in) (List.length rows_in); *)
    expansion_rate * ((List.length cols_in) + (List.length rows_in)) in
  let rec aux acc = function
    | [] -> acc
    | hd :: tl -> aux (List.fold_left ~init:acc ~f:(fun acc' other -> acc' + (manhattan_dist hd other) + (expansion_shift_in hd other)) tl) tl in
  aux 0 galaxies

let solve (_, galaxies, empty) =
  Printf.printf "Part1 - sum of distances (expansion_rate=2): %d\n" (compute_distances ~galaxies ~empty ~expansion_rate:1);
  Printf.printf "Part2 - sum of distances (expansion_rate=1000000): %d\n" (compute_distances ~galaxies ~empty ~expansion_rate:999999)

let main input = In_channel.read_all input |> parse |> solve
