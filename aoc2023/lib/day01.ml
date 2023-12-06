module List' = List
module String' = String
open Core

let digit_mappings =
  [
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

let double_stfoe str =
  let double_if_needed = function
    | ('s' | 't' | 'f' | 'o' | 'e') as c -> String.make 2 c
    | _ as c -> String.make 1 c
  in
  String.concat ~sep:"" (List.map ~f:double_if_needed (String.to_list str))

let parse_calibration_value line =
  double_stfoe line
  |> (fun x -> List.fold_left ~f:replace_word ~init:x digit_mappings)
  |> String.to_list |> List.filter ~f:Char.is_digit
  |> (fun x -> (List'.hd x, List.rev x |> List'.hd))
  |> (fun (a, b) -> String.make 1 a ^ String.make 1 b)
  |> int_of_string

let solve lines =
  List.map ~f:parse_calibration_value lines |> List.fold_left ~f:( + ) ~init:0

let main input =
  In_channel.read_lines input |> solve |> Printf.printf "Result: %d\n"
