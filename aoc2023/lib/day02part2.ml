module StringMap = Map.Make (String)
open Core
open Utils

let sum_by_identifier lst =
  let update_map acc (value, key) =
    let current_sum =
      match StringMap.find_opt key acc with Some sum -> sum | None -> 0
    in
    StringMap.add key (current_sum + value) acc
  in
  List.fold_left ~f:update_map ~init:StringMap.empty lst |> StringMap.bindings

let opt_to_int = function Some x -> x | None -> 0

let find_max_values lst =
  let update_max acc (color, value) =
    let current_max =
      match StringMap.find_opt color acc with Some max -> max | None -> value
      (* If it's the first time we see this color, it's the current maximum *)
    in
    StringMap.add color (max current_max value) acc
  in
  List.fold_left ~f:update_max ~init:StringMap.empty lst

(*PART2*)
let power_of_game game_spec =
  let strmap_opt_to_int key map = opt_to_int (StringMap.find_opt key map) in
  let max_values =
    String.split ~on:';' game_spec
    |> List.map ~f:String.strip
    |> List.map ~f:(String.split ~on:',')
    |> List.map
         ~f:
           (List.map ~f:(fun x ->
                String.strip x |> String.split ~on:' ' |> UList.to_tuple_exn
                |> fun (id, color) -> (Int.of_string id, color)))
    |> List.map ~f:sum_by_identifier
    |> List.concat |> find_max_values
  in
  strmap_opt_to_int "red" max_values
  * strmap_opt_to_int "blue" max_values
  * strmap_opt_to_int "green" max_values

let solve lines =
  lines
  |> List.map ~f:(fun line ->
         let game_spec =
           String.split ~on:':' line |> List.map ~f:String.strip |> List.tl_exn
           |> List.hd_exn |> String.strip
         in
         power_of_game game_spec)
  |> UList.fold_sum

let main input =
  In_channel.read_lines input |> solve |> Printf.printf "Result: %d\n"
