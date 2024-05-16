open Core

let with_borders lines =
  let len = List.hd_exn lines |> String.length in
  let dots = String.make (len + 2) '.' in
  List.map ~f:(fun x -> "." ^ x ^ ".") lines |> fun x -> dots :: (x @ [ dots ])

let solve lines =
  let width = List.hd_exn lines |> String.length in
  let height = List.length lines in
  let lines = with_borders lines in
  let i = ref 1 in
  let sum = ref 0 in
  let check_bounds str_len i j =
    let prev_line = List.nth_exn lines (i - 1) |> String.to_list in
    let curr_line = List.nth_exn lines i |> String.to_list in
    let next_line = List.nth_exn lines (i + 1) |> String.to_list in
    let chars =
      [
        List.nth_exn curr_line (j - 1);
        List.nth_exn curr_line (j + str_len);
        List.nth_exn prev_line (j - 1);
        List.nth_exn next_line (j - 1);
        List.nth_exn prev_line (j + str_len);
        List.nth_exn next_line (j + str_len);
      ]
      @ (List.range j (j + str_len)
        |> List.map ~f:(fun x ->
               [ List.nth_exn prev_line x; List.nth_exn next_line x ])
        |> List.concat)
    in
    List.exists chars ~f:(fun x -> Char.(x <> '.') && not (Char.is_alphanum x))
  in
  while !i <= width do
    let j = ref 1 in
    while !j <= height do
      let line = List.nth_exn lines !i |> String.to_list in
      sum :=
        !sum
        +
        if List.nth_exn line !j |> Char.is_digit then (
          let num_str =
            List.slice line !j (width + 1)
            |> List.take_while ~f:Char.is_digit
            |> String.of_char_list
          in
          let num_len = String.length num_str in
          j := !j + num_len;
          if check_bounds num_len !i (!j - num_len) then Int.of_string num_str
          else 0)
        else 0;
      j := !j + 1
    done;
    i := !i + 1
  done;
  !sum

let main input =
  In_channel.read_lines input |> solve |> Printf.printf "Result: %d\n"

let%test "Day03 - Test example input" =
  solve
    [
      "467..114..";
      "...*......";
      "..35..633.";
      "......#...";
      "617*......";
      ".....+.58.";
      "..592.....";
      "......755.";
      "...$.*....";
      ".664.598..";
    ]
  = 4361
