open Core

type node_type =
  | Galaxy
  | EmptySpace
[@@deriving sexp, compare, hash]

let node_type_of_char = function
  | '.' -> EmptySpace
  | '#' -> Galaxy
  | _ -> failwith "Unknown node type"

let int_of_node_type = function
  | Galaxy -> 1
  | EmptySpace -> 0

let node_type_of_int = function
  | 1 -> Galaxy
  | 0 -> EmptySpace
  | _ -> failwith "Unknown node type"

let char_of_node_type = function
  | Galaxy -> '#'
  | EmptySpace -> '.'

let parse input =
  let double_rows mat =
    let repeatNtimes elem n acc = List.init n ~f:(fun _ -> elem) @ acc in
    let rec aux acc = function
      | [] -> acc
      | hd :: tl -> if (List.fold_left ~init:0 ~f:(+) hd) = 0 then aux (repeatNtimes hd 1000000 acc) tl else aux (hd :: acc) tl in
    aux [] mat |> List.rev in
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
           |> List.map ~f:node_type_of_char
           |> List.map ~f:int_of_node_type)
    |> double_rows |> transpose |> double_rows |> transpose
  in
  let galaxies = ref [] in
  List.iteri ~f:(fun y row ->
      List.iteri ~f:(fun x node ->
          if node = 1 then galaxies := (x, y) :: !galaxies;) row) space;
  (space, !galaxies)

let manhattan_dist (x1, y1) (x2, y2) = abs (x1 - x2) + abs (y1 - y2)

let solve (space, galaxies) =
  let rec aux acc = function
    | [] -> acc
    | hd :: tl -> aux (List.fold_left ~init:acc ~f:(fun acc' other -> acc' + (manhattan_dist hd other)) tl) tl in
  Printf.printf "Part1 - sum of distances: %d\n" (aux 0 galaxies);
  Printf.printf "Result: %d" (List.length space + List.length galaxies)

let main input = In_channel.read_all input |> parse |> solve
