module StringMap = Map.Make(String)

let parse_game_id s =
  let prefix = "Game " in
  let number_str =
    String.sub s (String.length prefix) (String.length s - String.length prefix)
  in
  int_of_string number_str

let split_str separator str = Str.split (Str.regexp_string separator) str

let sum_by_identifier lst =
  let update_map acc (value, key) =
    let current_sum =
      match StringMap.find_opt key acc with Some sum -> sum | None -> 0
    in
    StringMap.add key (current_sum + value) acc
  in
  List.fold_left update_map StringMap.empty lst |> StringMap.bindings

let check_max_cubes = function
  | "red", n -> n <= 12
  | "green", n -> n <= 13
  | "blue", n -> n <= 14
  | _ -> false

let is_game_valid game_spec =
  split_str ";" game_spec |> List.map String.trim
  |> List.map (split_str ",")
  |> List.map
       (List.map (fun x ->
            split_str " " x |> fun y ->
            (List.hd y |> int_of_string, List.tl y |> List.hd)))
  |> List.map sum_by_identifier
  |> List.map (List.map check_max_cubes)
  |> List.map (List.fold_left ( && ) true)
  |> List.fold_left ( && ) true

let solve lines =
  lines |> List.map (fun line ->
      let game_id_str, game_spec =
        split_str ":" line |> fun x ->
        (List.hd x, List.tl x |> List.hd |> String.trim)
      in
      let game_id = parse_game_id game_id_str in
      if is_game_valid game_spec then game_id else 0
    ) |> List.fold_left ( + ) 0


let main input = Core.In_channel.read_lines input |> solve |> Printf.printf "Result: %d\n"
