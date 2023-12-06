module List' = List
module String' = String
module StringMap = Map.Make (String)
open Core

let parse_game_id s =
  let prefix = "Game " in
  let number_str =
    String'.sub s (String'.length prefix)
      (String'.length s - String'.length prefix)
  in
  Int.of_string number_str

let sum_by_identifier lst =
  let update_map acc (value, key) =
    let current_sum =
      match StringMap.find_opt key acc with Some sum -> sum | None -> 0
    in
    StringMap.add key (current_sum + value) acc
  in
  List.fold_left ~f:update_map ~init:StringMap.empty lst |> StringMap.bindings

let check_max_cubes = function
  | "red", n -> n <= 12
  | "green", n -> n <= 13
  | "blue", n -> n <= 14
  | _ -> false

let is_game_valid game_spec =
  String.split ~on:';' game_spec
  |> List.map ~f:String.strip
  |> List.map ~f:(String.split ~on:',')
  |> List.map
       ~f:
         (List.map ~f:(fun x ->
              String.strip x |> String.split ~on:' ' |> Utils.list_to_tuple
              |> fun (id, color) -> (Int.of_string id, color)))
  |> List.map ~f:sum_by_identifier
  |> List.map ~f:(List.map ~f:check_max_cubes)
  |> List.map ~f:(List.fold_left ~f:( && ) ~init:true)
  |> List.fold_left ~f:( && ) ~init:true

let solve lines =
  List.map lines ~f:(fun line ->
      let game_id_str, game_spec =
        String.split ~on:':' line |> List.map ~f:String.strip
        |> Utils.list_to_tuple
      in
      let game_id = parse_game_id game_id_str in
      if is_game_valid game_spec then game_id else 0)
  |> List.fold_left ~f:( + ) ~init:0

let main input =
  In_channel.read_lines input |> solve |> Printf.printf "Result: %d\n"
