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
  String.concat ""
    (List.map double_if_needed (String.to_seq str |> List.of_seq))

let is_digit c = c >= '0' && c <= '9'

let parse_calibration_value line =
  double_stfoe line |> fun x ->
  List.fold_left replace_word x digit_mappings
  |> String.to_seq |> List.of_seq |> List.filter is_digit
  |> fun x ->
  (List.hd x, List.rev x |> List.hd) |> fun (a, b) ->
  String.make 1 a ^ String.make 1 b |> int_of_string

let parse_document filename =
  Core.In_channel.read_lines filename
  |> List.map parse_calibration_value
  |> List.fold_left ( + ) 0


let main input = parse_document input |> Printf.printf "Solution: %d\n"
