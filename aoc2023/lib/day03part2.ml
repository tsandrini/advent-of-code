module List' = List

open Core

let with_borders lines =
  let len = List'.hd lines |> String.length in
  let dots = String.make (len + 2) '.' in
  List.map ~f:(fun x -> "." ^ x ^ ".") lines |> fun x -> dots :: (x @ [ dots ])

type num = { value : int; i : int; j : int; str_len : int }

let is_num_in_bounds n i j =
  let point_in_bounds x y =
    (x >= i - 1 && x <= i + 1) && y >= j - 1 && y <= j + 1
  in
  List.exists
    ~f:(fun x -> point_in_bounds n.i (n.j + x))
    (List.range 0 n.str_len)

let solve lines =
  let width = List'.hd lines |> String.length in
  let height = List.length lines in
  let lines = with_borders lines in
  let i = ref 1 in
  let sum = ref 0 in
  let nums = ref [] in
  (* First we parse the nums *)
  while !i <= width do
    let j = ref 1 in
    while !j <= height do
      let line = List'.nth lines !i |> String.to_list in

      if List'.nth line !j |> Char.is_digit then (
        let num_str =
          List.slice line !j (width + 1)
          |> List.take_while ~f:Char.is_digit
          |> String.of_char_list
        in
        nums :=
          {
            value = int_of_string num_str;
            i = !i;
            j = !j;
            str_len = String.length num_str;
          }
          :: !nums;
        j := !j + String.length num_str);
      j := !j + 1
    done;
    i := !i + 1
  done;
  i := 1;
  (*Having the nums now we parse the gears*)
  while !i <= width do
    let j = ref 1 in
    while !j <= height do
      let line = List'.nth lines !i |> String.to_list in
      (if Char.(List'.nth line !j = '*') then
         let nums_in_bounds =
           List.filter !nums ~f:(fun x -> is_num_in_bounds x !i !j)
         in
         sum :=
           !sum
           +
           if List.length nums_in_bounds > 1 then
             List.fold_left ~init:1
               ~f:(fun acc x -> acc * x.value)
               nums_in_bounds
           else 0);
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
  = 467835
