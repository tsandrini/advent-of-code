#!/usr/bin/env nix-shell
(*
#!nix-shell --pure -i ocaml -p ocaml
*)
#load "str.cma"
open Str
module StringMap = Map.Make(String)

let split_str separator str = Str.split (Str.regexp_string separator) str

let sum_by_identifier lst =
  let update_map acc (value, key) =
    let current_sum =
      match StringMap.find_opt key acc with Some sum -> sum | None -> 0
    in
    StringMap.add key (current_sum + value) acc
  in
  List.fold_left update_map StringMap.empty lst |> StringMap.bindings

let opt_to_int = function Some x -> x | None -> 0

let find_max_values lst =
  let update_max acc (color, value) =
    let current_max =
      match StringMap.find_opt color acc with Some max -> max | None -> value
      (* If it's the first time we see this color, it's the current maximum *)
    in
    StringMap.add color (max current_max value) acc
  in
  List.fold_left update_max StringMap.empty lst

(*PART2*)
let power_of_game game_spec =
  let strmap_opt_to_int key map = opt_to_int (StringMap.find_opt key map) in
  let max_values =
    split_str ";" game_spec |> List.map String.trim
    |> List.map (split_str ",")
    |> List.map
         (List.map (fun x ->
              split_str " " x |> fun y ->
              (List.hd y |> int_of_string, List.tl y |> List.hd)))
    |> List.map sum_by_identifier |> List.flatten |> find_max_values
  in
  strmap_opt_to_int "red" max_values
  * strmap_opt_to_int "blue" max_values
  * strmap_opt_to_int "green" max_values

let parse_input filename =
  let ic = open_in filename in
  let values = ref [] in
  try
    while true do
      let line = input_line ic in
      let _, game_spec =
        split_str ":" line |> fun x ->
        (List.hd x, List.tl x |> List.hd |> String.trim)
      in
      values := power_of_game game_spec :: !values
    done;
    []
  with
  | End_of_file ->
      close_in ic;
      !values
  | e ->
      close_in_noerr ic;
      raise e
;;

parse_input Sys.argv.(1) |> List.fold_left ( + ) 0 |> print_int;
print_endline
