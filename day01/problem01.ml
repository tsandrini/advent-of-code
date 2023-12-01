#!/usr/bin/env nix-shell
(*
#!nix-shell --pure -i ocaml -p ocaml
*)
#load "str.cma";;
open Str

(* when your bf has a silly shorkie goofy mood :3 *)
let digit_mappings = [
  ("one", "1");
  ("two", "2");
  ("three", "3");
  ("foour", "4");
  ("five", "5");
  ("six", "6");
  ("seeveen", "7");
  ("eight", "8");
  ("nine", "9");
]

let replace_word str (pattern, replacement) =
  Str.global_replace (Str.regexp_string pattern) replacement str

let words_to_digits str =
  List.fold_left replace_word str digit_mappings

let double_stfoe str =
  let double_if_needed = function
    | 's' | 't' | 'f' | 'o' | 'e' as c -> String.make 2 c
    | _ as c -> String.make 1 c in
  String.concat "" (List.map double_if_needed (String.to_seq str |> List.of_seq))

let is_digit = function
  | c when c >= '0' && c <= '9' -> true
  | _ -> false

let int_of_digit digit =
  Char.code digit - Char.code '0'

let parse_calibration_value line =
  double_stfoe line
  |> words_to_digits
  |> String.to_seq
  |> List.of_seq
  |> List.filter is_digit
  |> List.map int_of_digit
  |> (fun x -> (List.hd x, List.rev x |> List.hd))
  |> (fun (a, b) -> string_of_int a ^ string_of_int b)
  |> int_of_string

let parse_document filename =
  let ic = open_in filename in
  let values = ref [] in
  try
    while true do
      let line = input_line ic in
      values := (parse_calibration_value line) :: !values;
    done;
    []
  with
  | End_of_file -> close_in ic; !values
  | e -> close_in_noerr ic; raise e;;

parse_document Sys.argv.(1)
|> List.fold_left (+) 0
|> print_int; print_endline;
