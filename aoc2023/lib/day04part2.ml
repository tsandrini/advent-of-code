module List' = List
open Core

let replace_multiple_whitespaces str =
  Str.global_replace (Str.regexp "[ \t\n\r]+") " " str


let process_spec spec num_games =
  let counter = Array.init num_games ~f:(fun _ -> 1) in
  for i = 0 to (num_games - 1) do
    let spec = List'.nth spec i in
    List.iter spec ~f:(fun el -> counter.(el) <- counter.(el) + counter.(i))
  done;
  List.fold_left (Array.to_list counter) ~init:0 ~f:(+)

let parse_spec lines =
  let num_games = List.length lines in
  let specs = ref [] in
  for i = 0 to (num_games - 1) do
    let line = List'.nth lines i in
    let spec = line |> String.split ~on:':' |> List'.tl |> List'.hd |> String.strip
      |> replace_multiple_whitespaces |> String.split ~on:'|'
      |> List.map ~f:String.strip
      |> List.map ~f:(String.split ~on:' ')
      |> List.map ~f:(List.map ~f:Int.of_string)
      |> (fun x -> match x with [ a; b ] -> (a, b) | _ -> ([], []))
      |> (fun (x, y) -> List.filter ~f:(fun el -> List.mem x el ~equal:( = )) y)
      |> List.length |> List.range 0 |> List.map ~f:(fun x -> x + i + 1)
      |> List.filter ~f:(fun x -> x <= num_games)
    in
    specs := spec :: !specs
  done;
    List.rev !specs

let solve lines =
  let num_games = List.length lines in
  let spec = parse_spec lines in
  process_spec spec num_games

let main input = In_channel.read_lines input |> solve |> Printf.printf "%d\n"

let%test "Day04 - test example input" =
  solve
    [
      "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
      "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19";
      "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1";
      "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83";
      "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36";
      "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
    ]
  = 30
