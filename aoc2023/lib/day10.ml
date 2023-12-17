open Core
open Utils

type grid_node_type =
  | Ground
  | Start
  | VerticalPipe
  | HorizontalPipe
  | UpRightBend
  | UpLeftBend
  | LeftDownBend
  | RightDownBend
[@@deriving sexp, compare]

let grid_node_type_of_char = function
  | '.' -> Ground
  | 'S' -> Start
  | '|' -> VerticalPipe
  | '-' -> HorizontalPipe
  | 'L' -> UpRightBend
  | 'J' -> UpLeftBend
  | '7' -> LeftDownBend
  | 'F' -> RightDownBend
  | _ -> failwith "Invalid grid node type"

module IntPair = struct
  type t = int * int [@@deriving sexp, compare, hash]
end

let parse input =
  let tbl = Hashtbl.create (module IntPair) in
  let lines = String.split_lines input in
  let height = List.length lines in
  let width = String.length (List.hd_exn lines) in
  let y = ref 0 in
  let x = ref 0 in
  let start_pos = ref (0, 0) in
  List.iter lines ~f:(fun line ->
      String.iter line ~f:(fun c ->
          let node_type = grid_node_type_of_char c in
          Hashtbl.set tbl ~key:(!x, !y) ~data:node_type;
          if compare_grid_node_type node_type Start = 0 then
            start_pos := (!x, !y);
          x := !x + 1);
      y := !y + 1;
      x := 0);
  (tbl, !start_pos, (width, height))

let next_node_for ~prev_node:(x', y') ~curr_node:(x, y) ~dim:(width, height) =
  let xdiff = x - x' in
  let ydiff = y - y' in
  let m (a, b) =
    if a < 0 || a >= width || b < 0 || b >= height then None else Some (a, b)
  in
  function
  | VerticalPipe -> m (x, y + ydiff)
  | HorizontalPipe -> m (x + xdiff, y)
  | UpRightBend -> m (x + ydiff, y + xdiff)
  | UpLeftBend -> m (x - ydiff, y - xdiff)
  | LeftDownBend -> m (x + ydiff, y + xdiff)
  | RightDownBend -> m (x - ydiff, y - xdiff)
  | _ -> None

let init_start tbl ~start_pos:(x, y) =
  let left =
    match Hashtbl.find tbl (x - 1, y) with
    | Some HorizontalPipe | Some UpRightBend | Some RightDownBend ->
        Some (x - 1, y)
    | _ -> None
  in
  let right =
    match Hashtbl.find tbl (x + 1, y) with
    | Some HorizontalPipe | Some UpLeftBend | Some LeftDownBend ->
        Some (x + 1, y)
    | _ -> None
  in
  let up =
    match Hashtbl.find tbl (x, y - 1) with
    | Some VerticalPipe | Some LeftDownBend | Some RightDownBend ->
        Some (x, y - 1)
    | _ -> None
  in
  let down =
    match Hashtbl.find tbl (x, y + 1) with
    | Some VerticalPipe | Some UpLeftBend | Some UpRightBend -> Some (x, y + 1)
    | _ -> None
  in
  [ left; right; up; down ]
  |> List.filter ~f:(fun x -> match x with Some (_, _) -> true | _ -> false)
  |> List.map ~f:option_unwrap

let find_cycle tbl ~start_pos ~dim:(width, height) =
  let loop = ref true in
  let i = ref 0 in
  let init_nodes = init_start tbl ~start_pos in
  let curr_node = ref (List.nth_exn init_nodes !i) in
  let prev_node = ref start_pos in
  let cycle = ref [ !curr_node ] in
  while !loop do
    let curr_node_type = Hashtbl.find_exn tbl !curr_node in
    let next =
      next_node_for ~prev_node:!prev_node ~curr_node:!curr_node
        ~dim:(width, height) curr_node_type
    in

    if compare_grid_node_type curr_node_type Start = 0 then loop := false
    else if is_some next then (
      prev_node := !curr_node;
      curr_node := option_unwrap next;
      cycle := !curr_node :: !cycle)
    else (
      i := !i + 1;
      curr_node := List.nth_exn init_nodes !i;
      (*TODO error handling*)
      cycle := [ start_pos; !curr_node ])
  done;
  !cycle

(*TODO sad anime protagonist flashbacks*)
(* let lebesgue_integration_of_cycle cycle = *)
(*   let paired = Hashtbl.create (module IntPair) in *)
(*   let sum = ref 0 in *)
(*   let cycle_rest = ref cycle in *)
(*   let shortest = ref 0 in *)
(*   let shortest_path = ref 0 in *)
(*   List.iter cycle ~f:(fun (x, y) -> *)
(*       cycle_rest := List.tl_exn !cycle_rest; *)
(*       if not (Hashtbl.mem paired (x, y)) then ( *)
(*         shortest := 0; *)
(*         shortest_path := Int.max_value; *)
(*         List.iter !cycle_rest ~f:(fun (x', y') -> *)
(*             if x = x' && abs (y - y') < !shortest_path then ( *)
(*               shortest := y'; *)
(*               shortest_path := abs (y - y'))); *)
(*         if !shortest_path <> Int.max_value then ( *)
(*           Hashtbl.set paired ~key:(x, !shortest) ~data:(); *)
(*           sum := !sum + !shortest_path)); *)
(*   !sum *)

let shoelace_area coords =
  let rec aux acc = function
    | (x1, y1) :: (x2, y2) :: rest ->
        aux (acc + ((x1 * y2) - (x2 * y1))) ((x2, y2) :: rest)
    | _ -> acc
  in
  let closed_coords = coords @ [ List.hd_exn coords ] in
  abs (aux 0 closed_coords / 2)

let solve (tbl, start_pos, (width, height)) =
  let longest_cycle = find_cycle tbl ~start_pos ~dim:(width, height) in
  let cycle_integral = shoelace_area longest_cycle in
  let cycle_len = List.length longest_cycle / 2 in
  Printf.printf "Part1: Len(longest_cycle)/2 = %d\n" cycle_len;
  Printf.printf "Part2: Integral of longest cycle = %d\n"
    (cycle_integral - cycle_len + 1)

let main input = In_channel.read_all input |> parse |> solve
